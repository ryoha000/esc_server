use actix_web::{web, Error, HttpResponse, http};
use serde::{Deserialize, Serialize};
use super::super::actions::users;
use super::super::actions::randomids;
use super::super::actions::reviews;
use super::super::actions::logics::{es_login};
use super::super::actions::logics::scraping;
use super::super::models;
use super::super::middleware;
use std::ops::DerefMut;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostLogin {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditUser {
    pub user: models::User,
}

pub async fn me(
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

    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        let user = web::block(move || users::find_user_by_uid(me.user_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        if let Some(user) = user {
            return Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::NotFound()
                .body("No user found");
            return Ok(res)
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}

pub async fn get_user(
    pools: web::Data<super::super::Pools>,
    user_uid: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let search_user: models::User;
    match web::block(move || randomids::get_user_by_id(user_uid, &conn)).await {
        Ok(user) => search_user = user,
        _ => return Ok(HttpResponse::NotFound().body("user not found"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let user = web::block(move || users::find_user_by_uid(search_user.id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body("No user found");
        Ok(res)
    }
}

pub async fn get_users(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || users::find_users(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn signup(
    pools: web::Data<super::super::Pools>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    // println!("{}", &form.)

    let mut res = HttpResponse::new(http::StatusCode::OK);
    // Login処理
    if let Ok(cookie) = es_login::es_login(&form.name, &form.password)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            res = HttpResponse::NotFound()
                .body(format!("{:?}", e));
        }) {
            let mut new_user = models::User::new();
            new_user.es_user_id = form.name.clone();
            new_user.display_name = form.display_name.clone();
            let user = web::block(move || users::insert_new_user(new_user, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            
            
            let session_id = uuid::Uuid::new_v4().to_string();
            let mut redis_conn = pools.redis.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;
            
            // session_idとそのハッシュをRedisに、valueはそれぞれcookieとuser_id
            r2d2_redis::redis::pipe()
                .cmd("SET").arg(&format!("session_user:{}", session_id)).arg(user.id.clone())
                .cmd("SET").arg(&format!("session_header:{}", session_id)).arg(cookie.to_str().unwrap())
                .query(redis_conn.deref_mut()).map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let insert_reviews = scraping::reviews::get_reviews_by_es_user_id(form.name.clone())
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let _reviews = web::block(move || reviews::insert_new_reviews(insert_reviews, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
            })?;

            res = HttpResponse::Ok().header("set-cookie", format!("session_id={}", session_id)).json(user);
        }

    Ok(res)
}

pub async fn login(
    pools: web::Data<super::super::Pools>,
    form: web::Json<PostLogin>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let pass = form.password.clone();

    let user = web::block(move || users::find_user_by_name(form.name.clone(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    let mut res = HttpResponse::new(http::StatusCode::OK);
    if let Some(user) = user {
        // Login処理
        let cookie = es_login::es_login(&user.es_user_id, &pass)
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                res = HttpResponse::NotFound()
                    .body(format!("{:?}", e));
            })?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let mut redis_conn = pools.redis.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        // session_idとそのハッシュをRedisに、valueはそれぞれcookieとuser_id
        r2d2_redis::redis::pipe()
            .cmd("SET").arg(&format!("session_user:{}", session_id)).arg(user.id.clone())
            .cmd("SET").arg(&format!("session_header:{}", session_id)).arg(cookie.to_str().unwrap())
            .query(redis_conn.deref_mut()).map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        
        return Ok(HttpResponse::Ok().header("set-cookie", format!("session_id={}", session_id)).json(user))
    }

    Ok(HttpResponse::NotFound().body("user not found"))
}

pub async fn edit_user(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<EditUser>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        let uid = me.user_id.clone();
        let prev_user = web::block(move || users::find_user_by_uid(me.user_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(pu) = prev_user {
            if pu.es_user_id != form.user.es_user_id || uid != form.user.id {
                return Ok(HttpResponse::Forbidden().body("you are not this user"))
            }
        } else {
            return Ok(HttpResponse::NotFound().body("this user not found"))
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let user = web::block(move || users::update_user(uid, &form.user, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        
        return Ok(HttpResponse::Ok().json(user))
    } else {
        return Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}
