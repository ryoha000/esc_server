use actix_web::{web, Error, HttpResponse};
use super::super::middleware;
use super::super::actions::lists;
use super::super::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostList {
    pub name: String,
    pub comment: String,
}

pub async fn post_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<PostList>,
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
            let new_list = models::List::new(me.user_id, form.name.clone(), form.comment.clone());
            let _list = web::block(move || lists::insert_new_list(new_list, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            
            return Ok(HttpResponse::Ok().json(_list))
        },
        _ => {
            let res = HttpResponse::Unauthorized().body("Please login");
            return Ok(res)
        }
    }
}

pub async fn get_lists(
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

    let lists = web::block(move || lists::find_lists(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(lists))
}

pub async fn get_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    list_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let list_uid: uuid::Uuid = list_id.into_inner().parse().map_err(|_| {
        eprintln!("couldn't parse id to uuid");
        HttpResponse::InternalServerError().finish()
    })?;
    
    let list = web::block(move || lists::find_list_by_uid(list_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(list))
}
