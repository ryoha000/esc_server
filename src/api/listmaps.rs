use actix::prelude::*;
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
    srv: web::Data<Addr<super::super::ws_actor::WsActor>>,
    form: web::Json<AddGamesList>,
    list_id: web::Path<String>,
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

    match middleware::check_user(auth, &mut redis_conn) {
        Some(me) => {
            let list_id: String = list_id.into_inner();
            let list_uid: uuid::Uuid = list_id.parse().map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

            let list_with_game = web::block(move || lists::find_list_by_uid(list_uid, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let mut insert_games: Vec<i32> = Vec::new();
            match list_with_game.games {
                Some(games) => {
                    for in_g in &form.game_ids {
                        let mut exist = false;
                        for al_g in &games {
                            if &al_g.id == in_g {
                                exist = true;
                            }
                        }
                        if !exist {
                            insert_games.push(in_g.clone());
                        }
                    }
                },
                _ => insert_games = form.game_ids.clone()
            }

            match list_with_game.list {

                Some(_list) => {
                    if me.user_id != _list.user_id {
                        return Ok(HttpResponse::Forbidden().body("this request not allowed"))
                    }
                    
                    let mut new_listmaps: Vec<models::Listmap> = Vec::new();
                    for id in insert_games {
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

                    // 新しいTimeline の配列、フォローしてる人だけ見えるようにしようと思ったけどむずいからコメントアウト
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
                        ws_a.do_send(super::super::ws_actor::ClientMessage {
                            id: 0,
                            msg: _tl.id.clone(),
                        });
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

                    return Ok(HttpResponse::Created().body("game added"))
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

pub async fn delete_game_list(
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
            let list_id_clone = list_id.clone();

            let list = web::block(move || lists::find_simple_list_by_uid(list_id_clone, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            match list {
                Some(_list) => {
                    if me.user_id != _list.user_id {
                        return Ok(HttpResponse::Forbidden().body("this list owner is not you"))
                    }

                    let conn = pools.db.get().map_err(|_| {
                        eprintln!("couldn't get db connection from pools");
                        HttpResponse::InternalServerError().finish()
                    })?;

                    web::block(move || listmaps::delete_listmaps_by_list_id_and_list_map_ids(list_id, form.game_ids.clone(), &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;

                    return Ok(HttpResponse::Ok().body("games removed"))
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
