use actix_web::{web, Error, HttpResponse};
use super::super::middleware;
use super::super::actions::lists;
use super::super::actions::listmaps;
use super::super::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGamesList {
    pub game_ids: Vec<i32>,
}

pub async fn add_game_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<AddGamesList>,
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

    match middleware::check_user(auth, &mut redis_conn) {
        Some(me) => {
            let list_uid: uuid::Uuid = list_id.into_inner().parse().map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

            match web::block(move || lists::find_simple_list_by_uid(list_uid, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })? {

                Some(_list) => {
                    if me.user_id != _list.user_id {
                        return Ok(HttpResponse::Forbidden().body("this request not allowed"))
                    }
                    
                    let mut new_listmaps: Vec<models::Listmap> = Vec::new();
                    for id in form.game_ids.clone() {
                        let _listmap =  models::Listmap {
                            id: uuid::Uuid::new_v4().to_string(),
                            game_id: id,
                            list_id: list_uid.to_string(),
                        };
                        new_listmaps.push(_listmap);
                    }
        
                    let conn = pools.db.get().map_err(|_| {
                        eprintln!("couldn't get db connection from pools");
                        HttpResponse::InternalServerError().finish()
                    })?;
        
                    let _listmaps = web::block(move || listmaps::insert_new_listmaps(new_listmaps, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
        
                    return Ok(HttpResponse::Ok().json(_listmaps))
                },
                _ => {
                    let res = HttpResponse::NotFound().body("not found");
                    return Ok(res)
                }
            }

        },
        _ => {
            let res = HttpResponse::Unauthorized().body("Please login");
            return Ok(res)
        }
    }
}

pub async fn get_listmaps(
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

    let listmaps = web::block(move || listmaps::find_listmaps(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(listmaps))
}
