use actix_web::{web, Error, HttpResponse};
use std::ops::DerefMut;
use super::super::middleware;
use super::super::actions::timelines;

pub async fn post_play(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut user_id = String::from("");
    println!("{:?}", auth.session_id);
    if let Some(session_id) = auth.session_id {
        println!("{}", session_id);
        let header: String = r2d2_redis::redis::cmd("GET").arg(&format!("session_header:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        user_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    }

    let new_timeline = super::super::models::Timeline::new(user_id, game_id.into_inner(), 0);
    // use web::block to offload blocking Diesel code without blocking server thread
    let _timeline = web::block(move || timelines::insert_new_timeline(new_timeline, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(_timeline))
}
