use actix_web::{web, Error, HttpResponse};
use super::super::middleware;
use super::super::models;
use super::super::actions::follows;
use super::super::actions::randomids;
use super::super::actions::logics::mask;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct FollowWithUser {
    pub follow: models::Follow,
    pub user: models::User,
}

pub async fn post_follows(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    follower_id: web::Path<uuid::Uuid>,
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
        let follower: models::User;
        match web::block(move || randomids::get_user_by_id(follower_id.into_inner(), &conn)).await {
            Ok(user) => follower = user,
            _ => return Ok(HttpResponse::NotFound().body("user not found"))
        }

        if me.user_id == follower.id {
            return Ok(HttpResponse::BadRequest().body("follow user is you"))
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        // 既に対応されてないフォロリクあるなら400
        let fid = follower.id.clone();
        let my_id = me.user_id.clone();
        let follow_requests = web::block(move || follows::get_undeleted_follows_by_followee_id_and_follower_id(me.user_id, fid, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(frs) = follow_requests {
            if frs.len() != 0 {
                return Ok(HttpResponse::BadRequest().body("you are already send follow request"))
            }
        }

        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let new_follows = super::super::models::Follow::new(my_id, follower.id);
        // use web::block to offload blocking Diesel code without blocking server thread
        let _follows = web::block(move || follows::insert_new_follow(new_follows, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    
        Ok(HttpResponse::Ok().json(_follows))
    } else {
        Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}

pub async fn get_followers(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let user_uuid: uuid::Uuid;
    match user_id.parse::<uuid::Uuid>() {
        Ok(u_uuid) => user_uuid = u_uuid,
        _ => return Ok(HttpResponse::BadRequest().body("please enter a uuid"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _user: models::User;
    match web::block(move || randomids::get_user_by_id(user_uuid.clone(), &conn)).await {
        Ok(user) => _user = user,
        _ => return Ok(HttpResponse::NotFound().body("user not found"))
    }

    let unmask_user_uuid: uuid::Uuid;
    match _user.id.parse::<uuid::Uuid>() {
        Ok(u_uuid) => unmask_user_uuid = u_uuid,
        _ => return Ok(HttpResponse::BadRequest().body("please enter a uuid"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(me) = middleware::check_user(auth, &mut redis_conn) {
        let _follows = web::block(move || follows::find_followers_by_uid(unmask_user_uuid, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(followers) = _follows {
            let mut is_follower = false;
            for _f in &followers {
                if _f.id == me.user_id {
                    println!("{}{}", _f.id, me.user_id);
                    is_follower = true;
                }
            }
            if _user.id == me.user_id {
                is_follower = true;
            }
            if !is_follower {
                return Ok(HttpResponse::Forbidden().body("you did not follow this user"))
            }

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let masked_users = web::block(move || mask::mask_users(followers, models::RandomPurpose::FFollow, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            return Ok(HttpResponse::Ok().json(masked_users))
        }
        let res: Vec<models::User> = Vec::with_capacity(0);
        Ok(HttpResponse::Ok().json(res))
    } else {
        Ok(HttpResponse::Unauthorized().body("Please login"))
    }
}

pub async fn get_followees(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let user_uuid: uuid::Uuid;
    match user_id.parse::<uuid::Uuid>() {
        Ok(u_uuid) => user_uuid = u_uuid,
        _ => return Ok(HttpResponse::BadRequest().body("please enter a uuid"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _user: models::User;
    match web::block(move || randomids::get_user_by_id(user_uuid.clone(), &conn)).await {
        Ok(user) => _user = user,
        _ => return Ok(HttpResponse::NotFound().body("user not found"))
    }

    let unmask_user_uuid: uuid::Uuid;
    match _user.id.parse::<uuid::Uuid>() {
        Ok(u_uuid) => unmask_user_uuid = u_uuid,
        _ => return Ok(HttpResponse::BadRequest().body("please enter a uuid"))
    }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _followees = web::block(move || follows::find_followees_by_uid(unmask_user_uuid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let optionnal_me = middleware::check_user(auth, &mut redis_conn);

    // mask処理
    if let Some(followees) = _followees {
        let necessary_mask_users: Vec<models::User>;
        let mut unnecessary_mask_users: Vec<models::User> = Vec::new();

        // maskする必要のあるUserと、必要のないUserで分離
        if let Some(me) = optionnal_me {
            // もし自分ならmaskせずにreturn
            if _user.id != me.user_id {
                return Ok(HttpResponse::Ok().json(followees))
            }
            // 自分がFollowしてるユーザーを取得
            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            let me_uuid: uuid::Uuid;
            match me.user_id.parse::<uuid::Uuid>() {
                Ok(u_uuid) => me_uuid = u_uuid,
                _ => return Ok(HttpResponse::BadRequest().body("bad request"))
            }

            let _my_followees = web::block(move || follows::find_followees_by_uid(me_uuid, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            match _my_followees {
                Some(my_followees) => {
                    let _tupple = mask::find_wanna_mask_userids(&me.user_id, &my_followees, followees);
                    let mut tmp_users: Vec<models::User> = Vec::new();
                    for tu in _tupple.0 {
                        let mut u = tu;
                        u.password = String::from("");
                        tmp_users.push(u);
                    }
                    necessary_mask_users = tmp_users;
                    unnecessary_mask_users = _tupple.1;
                },
                _ => {
                    necessary_mask_users = followees;
                }
            }
        } else {
            necessary_mask_users = followees;
        }

        // mask処理
        let conn = pools.db.get().map_err(|_| {
            eprintln!("couldn't get db connection from pools");
            HttpResponse::InternalServerError().finish()
        })?;

        let mut masked_users = web::block(move || mask::mask_users(necessary_mask_users, models::RandomPurpose::FFollow, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        unnecessary_mask_users.append(&mut masked_users);

        return Ok(HttpResponse::Ok().json(unnecessary_mask_users))
    } else {
        Ok(HttpResponse::BadRequest().body("bad request"))
    }
}

pub async fn get_follow_request(
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
            // フォローリクエストの取得
            let follow_requests = web::block(move || follows::get_unapprove_follows_follower_id(me.user_id, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let mut user_ids: Vec<String> = Vec::new();
            let mut res: Vec<FollowWithUser> = Vec::new();
            if let Some(frs) = &follow_requests {
                for fr in frs {
                    user_ids.push(fr.followee_id.clone());
                }

                let conn = pools.db.get().map_err(|_| {
                    eprintln!("couldn't get db connection from pools");
                    HttpResponse::InternalServerError().finish()
                })?;

                let masked_users = web::block(move || mask::mask_users_by_ids(user_ids, models::RandomPurpose::FFollow, &conn))
                    .await
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;

                for fr in frs {
                    let mut new_fr = fr.clone();
                    match masked_users.get(&new_fr.followee_id) {
                        Some(mu) => {
                            new_fr.followee_id = mu.id.clone();
                            res.push(FollowWithUser { follow: new_fr, user: mu.clone() });
                        },
                        _ => {}
                    }
                }
            }
            Ok(HttpResponse::Ok().json(res))
        },
        _ => {
            return Ok(HttpResponse::Unauthorized().body("Please login"))
        }
    }
}

pub async fn get_my_follow_request(
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
            // フォローリクエストの取得
            let follow_requests = web::block(move || follows::get_all_follows_followee_id(me.user_id, &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let mut user_ids: Vec<String> = Vec::new();
            let mut res: Vec<FollowWithUser> = Vec::new();
            if let Some(frs) = &follow_requests {
                for fr in frs {
                    user_ids.push(fr.follower_id.clone());
                }

                let conn = pools.db.get().map_err(|_| {
                    eprintln!("couldn't get db connection from pools");
                    HttpResponse::InternalServerError().finish()
                })?;

                let masked_users = web::block(move || mask::mask_users_by_ids(user_ids, models::RandomPurpose::FFollow, &conn))
                    .await
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;

                for fr in frs {
                    let mut new_fr = fr.clone();
                    match masked_users.get(&new_fr.follower_id) {
                        Some(mu) => {
                            new_fr.follower_id = mu.id.clone();
                            res.push(FollowWithUser { follow: new_fr, user: mu.clone() });
                        },
                        _ => {}
                    }
                }
            }
            Ok(HttpResponse::Ok().json(res))
        },
        _ => {
            return Ok(HttpResponse::Unauthorized().body("Please login"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approve: bool
}

pub async fn handle_follow_request(
    auth: middleware::Authorized,
    pools: web::Data<super::super::Pools>,
    follow_id: web::Path<uuid::Uuid>,
    approval: web::Json<Approval>,
) -> Result<HttpResponse, Error> {
    let follow_id = follow_id.into_inner();
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
            // フォローリクエストの取得
            let option_follow_request = web::block(move || follows::find_follow_by_uid(follow_id.clone(), &conn))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let follow_request: models::Follow;
            if let Some(fr) = option_follow_request {
                follow_request = fr;
                // もし申請元じゃないのにリクエスト飛ばしてたら弾く
                if follow_request.follower_id != me.user_id {
                    return Ok(HttpResponse::Forbidden().body("this method is not permitted"))
                }
            } else {
                return Ok(HttpResponse::NotFound().body("follow request is not found"))
            }

            let conn = pools.db.get().map_err(|_| {
                eprintln!("couldn't get db connection from pools");
                HttpResponse::InternalServerError().finish()
            })?;

            match approval.approve {
                true => {
                    let _ = web::block(move || follows::approve_follow(follow_id, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
                    return Ok(HttpResponse::Ok().body("accept follow"))
                },
                false => {
                    web::block(move || follows::delete_follow(follow_id, &conn))
                        .await
                        .map_err(|e| {
                            eprintln!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        })?;
                    return Ok(HttpResponse::Ok().body("remove follow request"))
                }
            }
        },
        _ => {
            return Ok(HttpResponse::Unauthorized().body("Please login"))
        }
    }
}
