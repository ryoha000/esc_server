use actix_web::{web, Error, HttpResponse};
use super::super::actions::logics::mask;
use super::super::models;
use super::super::middleware;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct MaskedTimeline {
    pub timeline: models::Timeline,
    pub review: Option<models::Review>,
    pub list: Option<models::List>,
    pub game: models::Game,
    pub user: models::User,
}

#[derive(Deserialize)]
pub struct GetInfo {
    pub offset: i64,
}

pub async fn get_timelines(
    info: web::Query<GetInfo>,
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let me = middleware::check_user(auth, &mut redis_conn);
    let _timelines = web::block(move || mask::mask_recent_timelines(me, info.offset, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(_timelines))
}

pub async fn get_timeline(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    timeline_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let me = middleware::check_user(auth, &mut redis_conn);
    match web::block(move || mask::mask_timeline(me, timeline_id.into_inner(), &conn)).await {
        Ok(get_timeline) => {
            Ok(HttpResponse::Ok().json(get_timeline))
        },
        _ => {
            Ok(HttpResponse::Forbidden().body("timeline is not found"))
        }
    }
}
