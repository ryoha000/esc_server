use actix_web::{web, Error, HttpResponse};
use std::ops::DerefMut;
use super::super::middleware;
use super::super::models;
use super::super::actions::messages;
use super::super::actions::randomids;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMessage {
    pub message: String,
}

pub async fn post_messages(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<PostMessage>,
    to_user_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    println!("po");
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
        let from_user_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        let to_user: models::User;
        match web::block(move || randomids::get_user_by_id(to_user_id.into_inner(), &conn)).await {
            Ok(user) => to_user = user,
            _ => return Ok(HttpResponse::NotFound().body("user not found"))
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let new_messages = super::super::models::Message::new(from_user_id, to_user.id, form.message.clone());
        // use web::block to offload blocking Diesel code without blocking server thread
        let _messages = web::block(move || messages::insert_new_message(new_messages, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        Ok(HttpResponse::Ok().json(_messages))
    } else {
        Ok(HttpResponse::Unauthorized().body("Please login"))
    }
    // Ok(HttpResponse::Ok().body("_messages"))
}

pub async fn get_messages(
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

    if let Some(session_id) = auth.session_id {
        let user_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        let _messages = web::block(move || messages::find_messages_by_to_user_id(user_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        Ok(HttpResponse::Ok().json(_messages))
    } else {
        Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}
