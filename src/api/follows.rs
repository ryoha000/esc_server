use actix_web::{web, Error, HttpResponse};
use std::ops::DerefMut;
use super::super::middleware;
use super::super::models;
use super::super::actions::follows;
use super::super::actions::randomids;

pub async fn post_follows(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    followee_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    println!("{:?}", auth.session_id);
    if let Some(session_id) = auth.session_id {
        let follower_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        let followee: models::User;
        match web::block(move || randomids::get_user_by_id(followee_id.into_inner(), &conn)).await {
            Ok(user) => followee = user,
            _ => return Ok(HttpResponse::NotFound().body("user not found"))
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let new_follows = super::super::models::Follow::new(followee.id, follower_id);
        // use web::block to offload blocking Diesel code without blocking server thread
        let _follows = web::block(move || follows::insert_new_follow(new_follows, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        Ok(HttpResponse::Ok().json(_follows))
    } else {
        Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}

pub async fn get_followers(
    pools: web::Data<super::super::Pools>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let _follows = web::block(move || follows::find_followers_by_uid(user_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(_follows))
}

pub async fn get_follow_request(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    match middleware::check_user(auth, &mut redis_conn) {
        Some(me) => {
            // フォローリクエストの取得
            let follow_requests = web::block(move || follows::get_unapprove_follows_follower_id(me.user_id, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            Ok(HttpResponse::Ok().json(follow_requests))
        },
        _ => {
            return Ok(HttpResponse::Unauthorized().body("Please login"))
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approve: bool
}

pub async fn handle_follow_request(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    follow_id: web::Path<uuid::Uuid>,
    approval: web::Json<Approval>,
) -> Result<HttpResponse, Error> {
    let follow_id = follow_id.into_inner();
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    match middleware::check_user(auth, &mut redis_conn) {
        Some(me) => {
            // フォローリクエストの取得
            let option_follow_request = web::block(move || follows::find_follow_by_uid(follow_id.clone(), &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let follow_request: models::Follow;
            if let Some(fr) = option_follow_request {
                follow_request = fr;
                // もし申請元じゃないのにリクエスト飛ばしてたら弾く
                if follow_request.follower_id != me.user_id {
                    return Ok(HttpResponse::Forbidden().body("this method is not permitted"))
                }
            } else {
                return Ok(HttpResponse::NotFound().body("follow request is not found"))
            }

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            match approval.approve {
                true => {
                    let _ = web::block(move || follows::approve_follow(follow_id, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
                    return Ok(HttpResponse::Ok().body("accept follow"))
                },
                false => {
                    let deleted_follow = web::block(move || follows::delete_follow(follow_id, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
                    return Ok(HttpResponse::Ok().json(deleted_follow))
                }
            }
        },
        _ => {
            return Ok(HttpResponse::Unauthorized().body("Please login"))
        }
    }
}
