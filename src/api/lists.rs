use actix_web::{web, Error, HttpResponse};
use super::super::middleware;
use super::super::actions::lists;
use super::super::actions;
use super::super::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostList {
    pub name: String,
    pub comment: String,
    pub priority: i32,
    pub url: Option<String>,
    pub is_public: bool,
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
            let new_list = models::List::new(me.user_id, form.name.clone(), form.comment.clone(), form.priority, form.url.clone(), form.is_public);
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

pub async fn put_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<PostList>,
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
            let _prev_list = web::block(move || lists::find_simple_list_by_uid(list_id.into_inner(), &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            if let Some(prev_list) = _prev_list {
                if prev_list.user_id != me.user_id {
                    return Ok(HttpResponse::Forbidden().body("this list owner is not you"))
                }
                let mut new_list = models::List::new(me.user_id, form.name.clone(), form.comment.clone(), form.priority, form.url.clone(), form.is_public);
                new_list.id = prev_list.id;
                new_list.created_at = prev_list.created_at;

                let conn = pools.db.get().map_err(|_| {
                    eprintln!("couldn't get db connection from pools");
                    HttpResponse::InternalServerError().finish()
                })?;

                let _list = web::block(move || lists::update_list_by_id(&new_list, &conn))
                    .await
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;
                
                return Ok(HttpResponse::Ok().json(_list.get(0)))
            }
            return Ok(HttpResponse::NotFound().body("list is not found"))
        },
        _ => {
            let res = HttpResponse::Unauthorized().body("Please login");
            return Ok(res)
        }
    }
}

pub async fn delete_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    list_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let list_str = list_id.into_inner();
    let list_str_clone = list_str.clone();
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
            let _list = web::block(move || lists::find_simple_list_by_uid(list_str, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            if let Some(list) = _list {
                if list.user_id == me.user_id {
                    let conn = pools.db.get().map_err(|_| {
                        eprintln!("couldn't get db connection from pools");
                        HttpResponse::InternalServerError().finish()
                    })?;

                    let deleted_lists = web::block(move || lists::delete_list_by_id(list_str_clone, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
                    return Ok(HttpResponse::Ok().json(deleted_lists))
                }
                Ok(HttpResponse::Forbidden().body("this list owner is not you"))
            } else {
                Ok(HttpResponse::NotFound().body("list is not found"))
            }
        },
        _ => return Ok(HttpResponse::Unauthorized().body("please login"))
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

    match middleware::check_user(auth, &mut redis_conn) {
        Some(me) => {
            let lists = web::block(move || lists::find_simple_lists_by_user_id(me.user_id, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            Ok(HttpResponse::Ok().json(lists))
        },
        _ => return Ok(HttpResponse::Unauthorized().body("please login"))
    }
}

pub async fn get_lists_by_user_id(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    user_id: web::Path<String>,
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
            let user_id = user_id.into_inner();
            let user_id_clone = user_id.clone();

            let lists = web::block(move || lists::find_simple_lists_by_user_id(user_id, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            // 自分なら何もせずに返却
            if user_id_clone == me.user_id {
                return Ok(HttpResponse::Ok().json(lists))
            }

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let me_uuid: uuid::Uuid = me.user_id.parse().map_err(|e| {
                HttpResponse::InternalServerError().finish()
            })?;

            let _followees = web::block(move || actions::follows::find_followees_by_uid(me_uuid, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let mut is_follow = false;
            let mut target_user = models::User::new();
            if let Some(followees) = _followees {
                for flee in &followees {
                    if flee.id == user_id_clone {
                        is_follow = true;
                        target_user = flee.clone();
                        break
                    }
                }
            }

            // followerじゃないならForbidden
            if !is_follow {
                return Ok(HttpResponse::Forbidden().body("you are not follow this user"))
            }

            // followerにも隠す設定ならForbidden
            match target_user.show_followers {
                Some(b) => {
                    if !b {
                        return Ok(HttpResponse::Forbidden().body("This user does not disclose information to their followers"))
                    }
                },
                _ => return Ok(HttpResponse::Forbidden().body("This user does not disclose information to their followers"))
            }

            let mut res_lists: Vec<models::List> = Vec::new();
            if let Some(unfilter_list) = lists {
                for li in unfilter_list {
                    // publicなものだけreturn
                    if li.is_public {
                        res_lists.push(li);
                    }
                }
            }

            Ok(HttpResponse::Ok().json(res_lists))
        },
        _ => return Ok(HttpResponse::Unauthorized().body("please login"))
    }
}

pub async fn get_list(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    list_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    
    let list = web::block(move || lists::find_list_by_uid(list_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(list))
}
