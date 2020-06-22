use actix_web::{web, Error, HttpResponse};
use uuid::Uuid;
use super::super::actions::timelines;
use super::super::models;
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct MaskedTimeline {
    pub timeline: models::Timeline,
    pub review_log_id: Uuid,
    pub list_log_id: Uuid,
    pub session_id: Uuid,
}

pub async fn get_timelines(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let _timelines = web::block(move || timelines::find_timelines(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(_timelines))
}