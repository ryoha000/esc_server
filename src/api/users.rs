use actix_web::{web, Error, HttpResponse, http};
use serde::{Deserialize, Serialize};
use super::super::actions::users;
use super::super::actions::randomids;
use super::super::actions::follows;
use super::super::actions::timelines;
use super::super::actions::reviews;
use super::super::actions::logics::{es_login};
use super::super::actions::logics::scraping;
use super::super::actions::logics::mask;
use super::super::models;
use super::super::middleware;
use std::ops::DerefMut;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostLogin {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditUser {
    pub user: models::User,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimelineWithGame {
    pub timeline: models::Timeline,
    pub game: models::Game,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseUserDetail {
    pub user: models::User,
    pub play: Vec<TimelineWithGame>,
    pub review: Vec<TimelineWithGame>,
    pub list: Vec<TimelineWithGame>,
}

pub async fn me(
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

    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        let user = web::block(move || users::find_user_by_uid(me.user_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        if let Some(user) = user {
            return Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::NotFound()
                .body("No user found");
            return Ok(res)
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}

pub async fn get_user(
    pools: web::Data<super::super::Pools>,
    user_uid: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let search_user: models::User;
    match web::block(move || randomids::get_user_by_id(user_uid, &conn)).await {
        Ok(user) => search_user = user,
        _ => return Ok(HttpResponse::NotFound().body("user not found"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let user = web::block(move || users::find_user_by_uid(search_user.id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body("No user found");
        Ok(res)
    }
}

pub async fn get_user_detail(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    user_uid: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let _user_uid =user_uid.clone();
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut search_user: models::User;
    let search_randomid: models::Randomid;
    match web::block(move || randomids::find_randomid_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })? {
        Some(rid) => search_randomid = rid,
        _ => return Ok(HttpResponse::NotFound().body("user not found"))
    }

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // maskの必要があるかどうか
    let mut is_follow = false;
    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        if _user_uid.to_string() == me.user_id {
            is_follow = true;
        } else {
            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let _user_followers = web::block(move || follows::find_followers_by_uid(_user_uid, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            if let Some(user_followers) = _user_followers {
                for uf in user_followers {
                    if uf.id == me.user_id {
                        is_follow = true;
                    }
                }
            }
        }
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // とりあえずuserを取得
    if !is_follow {
        let s_id = search_randomid.user_id.clone();
        search_user = web::block(move || mask::mask_user_by_id(s_id, models::RandomPurpose::FDirect, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        // どこからのアクセスかで匿名化の程度を変更
        match search_randomid.purpose {
            0 => {},
            2 => {},
            3 => {},
            7 => {},
            _ => {
                match &search_user.id[..] {
                    "" => {},
                    _ => {
                        search_user.display_name = String::from("名無しさん");
                        search_user.es_user_id = String::from("内緒");
                    },
                }
                search_user.icon_url = None;
                search_user.twitter_id = None;
            }
        }
    } else {
        match web::block(move || users::find_user_by_uid(_user_uid.to_string(), &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })? {
            Some(user) => search_user = user,
            _ => return Ok(HttpResponse::NotFound().body("user not found"))
        }
    }

    // 返却するTimelineの用意
    let mut play_timeline: Vec<TimelineWithGame> = Vec::new();
    let mut review_timeline: Vec<TimelineWithGame> = Vec::new();
    let mut list_timeline: Vec<TimelineWithGame> = Vec::new();
    
    let limit_num: i64;
    match is_follow {
        true => limit_num = 20,
        false=> limit_num = 3,
    }

    let set_tl = async {
        for i in 0..2 {
            if let Ok(conn) = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            }) {
                let s_id = search_randomid.user_id.clone();
                let mut _tl_with_g_vec_result = web::block(move || timelines::find_timelines_with_game_by_user_id_and_type_with_limit(s_id, i, limit_num, &conn))
                    .await
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    });
    
                if let Ok(tl_with_g_vec_option) = _tl_with_g_vec_result {
                    if let Some(tl_with_g_vec) = tl_with_g_vec_option {
                        for (tl, g) in tl_with_g_vec {
                            let mut new_tl = tl;
                            if !is_follow {
                                new_tl.user_id = search_user.id.clone();
                            }
                            if i == 0 {
                                play_timeline.push(TimelineWithGame { timeline: new_tl, game: g });
                            } else if i == 1 {
                                review_timeline.push(TimelineWithGame { timeline: new_tl, game: g });
                            } else if i == 2 {
                                list_timeline.push(TimelineWithGame { timeline: new_tl, game: g });
                            }
                        }
                    }
                }
            }
        
        }
    };
    set_tl.await;
    Ok(HttpResponse::Ok().json(ResponseUserDetail { user: search_user, play: play_timeline, review: review_timeline, list: list_timeline }))
}

pub async fn get_users(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || users::find_users(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn signup(
    pools: web::Data<super::super::Pools>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    // println!("{}", &form.)

    let mut res = HttpResponse::new(http::StatusCode::OK);
    // Login処理
    if let Ok(cookie) = es_login::es_login(&form.name, &form.password)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            res = HttpResponse::NotFound()
                .body(format!("{:?}", e));
        }) {
            let mut new_user = models::User::new();
            new_user.es_user_id = form.name.clone();
            new_user.display_name = form.display_name.clone();
            let user = web::block(move || users::insert_new_user(new_user, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            
            
            let session_id = uuid::Uuid::new_v4().to_string();
            let mut redis_conn = pools.redis.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;
            
            // session_idとそのハッシュをRedisに、valueはそれぞれcookieとuser_id
            r2d2_redis::redis::pipe()
                .cmd("SET").arg(&format!("session_user:{}", session_id)).arg(user.id.clone())
                .cmd("SET").arg(&format!("session_header:{}", session_id)).arg(cookie.to_str().unwrap())
                .query(redis_conn.deref_mut()).map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let insert_reviews = scraping::reviews::get_reviews_by_es_user_id(form.name.clone())
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let _reviews = web::block(move || reviews::insert_new_reviews(insert_reviews, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
            })?;

            res = HttpResponse::Ok().header("set-cookie", format!("session_id={}", session_id)).json(user);
        }

    Ok(res)
}

pub async fn login(
    pools: web::Data<super::super::Pools>,
    form: web::Json<PostLogin>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let pass = form.password.clone();

    let user = web::block(move || users::find_user_by_name(form.name.clone(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    let mut res = HttpResponse::new(http::StatusCode::OK);
    if let Some(user) = user {
        // Login処理
        let cookie = es_login::es_login(&user.es_user_id, &pass)
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                res = HttpResponse::NotFound()
                    .body(format!("{:?}", e));
            })?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let mut redis_conn = pools.redis.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        // session_idとそのハッシュをRedisに、valueはそれぞれcookieとuser_id
        r2d2_redis::redis::pipe()
            .cmd("SET").arg(&format!("session_user:{}", session_id)).arg(user.id.clone())
            .cmd("SET").arg(&format!("session_header:{}", session_id)).arg(cookie.to_str().unwrap())
            .query(redis_conn.deref_mut()).map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        
        return Ok(HttpResponse::Ok().header("set-cookie", format!("session_id={}", session_id)).json(user))
    }

    Ok(HttpResponse::NotFound().body("user not found"))
}

pub async fn edit_user(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    form: web::Json<EditUser>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        let uid = me.user_id.clone();
        let prev_user = web::block(move || users::find_user_by_uid(me.user_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(pu) = prev_user {
            if pu.es_user_id != form.user.es_user_id || uid != form.user.id {
                return Ok(HttpResponse::Forbidden().body("you are not this user"))
            }
        } else {
            return Ok(HttpResponse::NotFound().body("this user not found"))
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let users = web::block(move || users::update_user(uid, &form.user, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        
        return Ok(HttpResponse::Ok().json(users.get(0)))
    } else {
        return Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}
