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
        println!("{}", session_id);
        let header: String = r2d2_redis::redis::cmd("GET").arg(&format!("session_header:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        let follower_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        let mut followee: models::User;
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

    let mut redis_conn = pools.redis.get().map_err(|_| {
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
