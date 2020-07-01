use actix_web::{web, Error, HttpResponse};
use std::ops::DerefMut;
use std::collections::HashMap;
use super::super::actions::logics::scraping;

pub async fn set_campaigns(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _campaigns = scraping::campaign::get_now_campaign()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut pipe = r2d2_redis::redis::pipe();
    let cmd = pipe
        .cmd("DEL").arg("campaigns");

    for (key, value) in &_campaigns {
        println!("{}: {:?}", key, value);
        let v = serde_json::to_string(value).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        cmd.cmd("HSET").arg("campaigns").arg(*key).arg(v);
    }

    cmd.query(redis_conn.deref_mut()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(_campaigns))
}

pub async fn get_campaigns(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _campaigns: HashMap<i32, String> = r2d2_redis::redis::cmd("HGETALL")
        .arg("campaigns").query(redis_conn.deref_mut()).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut res: Vec<scraping::campaign::Campaign> = Vec::new();

    for (_, value) in &_campaigns {
        res.push(serde_json::from_str(value).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?);
    }

    Ok(HttpResponse::Ok().json(res))
}
