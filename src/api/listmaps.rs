use actix_web::{web, Error, HttpResponse};
use super::super::middleware;
use super::super::actions::lists;
use super::super::actions::listmaps;
use super::super::actions::timelines;
use super::super::actions::listlogs;
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
            let list_id: String = list_id.into_inner();
            let list_uid: uuid::Uuid = list_id.parse().map_err(|e| {
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

                    // 新しいTimeline の配列
                    let mut new_timelines: Vec<models::Timeline> = Vec::new();

                    let conn = pools.db.get().map_err(|_| {
                        eprintln!("couldn't get db connection from pools");
                        HttpResponse::InternalServerError().finish()
                    })?;

                    for _lm in _listmaps {
                        let new_timeline = models::Timeline::new(me.user_id.clone(), _lm.game_id, models::LogType::List as i32);
                        new_timelines.push(new_timeline);
                    }

                    let _timelines = web::block(move || timelines::insert_new_timelines(new_timelines, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;

                    let mut new_listlogs: Vec<models::Listlog> = Vec::new();

                    for _tl in _timelines {
                        new_listlogs.push(models::Listlog::new(_tl.id, list_id.clone()));
                    }

                    let conn = pools.db.get().map_err(|_| {
                        eprintln!("couldn't get db connection from pools");
                        HttpResponse::InternalServerError().finish()
                    })?;

                    let _ = web::block(move || listlogs::insert_new_listlogs(new_listlogs, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;

                    return Ok(HttpResponse::Created().body("ok"))
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
