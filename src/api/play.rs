use actix::prelude::*;
use actix_web::{web, Error, HttpResponse};
use std::ops::DerefMut;
use super::super::middleware;
use super::super::models;
use super::super::actions::timelines;

pub async fn post_play(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    srv: web::Data<Addr<super::super::ws_actor::WsActor>>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let ws_a = srv.get_ref().clone();

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut user_id = String::from("");
    if let Some(session_id) = auth.session_id {
        user_id = r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    }

    let new_timeline = models::Timeline::new(user_id, game_id.into_inner(), models::LogType::Play as i32);
    // use web::block to offload blocking Diesel code without blocking server thread
    let _timeline = web::block(move || timelines::insert_new_timeline(new_timeline, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    ws_a.do_send(super::super::ws_actor::ClientMessage {
        id: 0,
        msg: _timeline.id.clone(),
    });

    Ok(HttpResponse::Ok().json(_timeline))
}
