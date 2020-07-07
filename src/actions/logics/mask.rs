use super::super::super::models;
use super::super::super::middleware;
use super::super::super::api::timelines;
use super::super::super::actions;
use std::collections::HashMap;


use diesel::prelude::*;
use anyhow::{Context, Result};

// pub fn mask_timelines(
//     auth: middleware::Authorized,
//     timelines: Vec<models::Timeline>,
//     redis_pool: RedisPool,
//     conn: &PgConnection,
// ) -> Result<Vec<timelines::MaskedTimeline>> {
//     let mut redis_conn = redis_pool.get().context("couldn't get db connection from pools")?;

//     let followees = actions::follows::find_followees_by_uid(user_id, conn)?;
//     let mut new_timelines: Vec<timelines::MaskedTimeline> = Vec::new();

//     for _tl in timelines {
//         let mut new_tl = _tl;

//         let mut is_follow = false;
//         for flee in &followees {
//             if new_tl.user_id == flee.id {
//                 is_follow = true;
//             }
//         }

//         if !is_follow {
//             new_tl.user_id = String::from("");
//         }

//         let mut _review: Option<models::Review> = None;
//         match new_tl.log_type {
//             // Play => 0, Review => 1, List = 2
//             0 => 
//         }
//         // new_timelines.push(
//         //     timelines::MaskedTimeline {
//         //         timeline: new_tl
//         //     }
//         // );
//     }
//     Ok(new_timelines)
// }

pub fn mask_timeline(
    me: Option<middleware::Me>,
    timeline_id: String,
    conn: &PgConnection,
) -> Result<timelines::MaskedTimeline> {
    // 返答のためのstructの準備
    let mut res_tl: models::Timeline;
    let res_game: models::Game;
    if let Some((_timeline, _game)) = actions::timelines::find_timeline_with_game_by_timeline_id(timeline_id.clone(), conn)? {
        res_game = _game;
        res_tl = _timeline;
    } else {
        anyhow::bail!("timeline not found")
    }

    // 匿名化する場合のidを取得
    let random_id = actions::randomids::get_randomid_by_user_id(res_tl.user_id.clone(), models::RandomPurpose::FTimeline as i32, conn)?;

    let mut is_follow: bool = false;
    match me {
        Some(_me) => {
            let my_uuid: uuid::Uuid = _me.user_id.parse().context("please enter uuid")?;

            if let Some(followees) = actions::follows::find_followees_by_uid(my_uuid, conn)? {
                // action主のfollowerかどうか確認
                for flee in &followees {
                    if res_tl.user_id.clone() == flee.id {
                        is_follow = true;
                    }
                }
            }
        }
        _ => {}
    }

    let mut user = models::User::annonymus(random_id.id.clone(), String::from(""));
    match is_follow {
        true => {
            if let Some(getted_user) = actions::users::find_user_by_uid(res_tl.user_id.clone(), conn)? {
                user = getted_user;
            }
        },
        false => {
            res_tl.user_id = random_id.id.clone();
        }
    }

    // reviewかListを挿入
    let mut _review: Option<models::Review> = None;
    let mut _list: Option<models::List> = None;
    match res_tl.log_type.clone() {
        // Play => 0, Review => 1, List = 2
        1 => {
            if let Some((_reviewlog, found_review)) = actions::reviewlogs::find_review_by_timeline_id(timeline_id.clone(), conn)? {
                let mut assign_review = found_review;
                if !is_follow {
                    assign_review.user_id = random_id.id;
                }
                _review = Some(assign_review);
            }
        },
        2 => {
            if let Some((_listlog, found_list)) = actions::listlogs::find_list_by_timeline_id(timeline_id, conn)? {
                let mut assign_list = found_list;
                if !is_follow {
                    assign_list.user_id = random_id.id;
                }
                _list = Some(assign_list);
            }
        },
        _ => {}
    }

    // MaskedTimelineを返す
    Ok(timelines::MaskedTimeline {
        timeline: res_tl,
        list: _list,
        review: _review,
        game: res_game,
        user: user
    })
}

pub fn mask_user(
    input_user: models::User,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<models::User> {
    let get_random_id = actions::randomids::get_randomid_by_user_id(input_user.id.clone(), purpose as i32, conn)?;

    let _user = models::User::light_annonymus(get_random_id.id, input_user);
    Ok(_user)
}

pub fn mask_users(
    input_users: Vec<models::User>,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<Vec<models::User>> {
    let mut user_ids: Vec<String> = Vec::new();
    for new_user in &input_users {
        user_ids.push(new_user.id.clone());
    }
    let get_random_ids = actions::randomids::get_randomids_with_users_by_user_ids(user_ids, purpose as i32, conn)?;

    let mut _users: Vec<models::User> = Vec::new();
    for (r, u) in get_random_ids {
        _users.push(models::User::light_annonymus(r.id, u));
    }
    Ok(_users)
}

pub fn mask_user_by_id(
    user_id: String,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<models::User> {
    let random_id_with_user = actions::randomids::get_randomid_with_user_by_user_id(user_id, purpose as i32, conn)?;

    match random_id_with_user {
        Some((rid, u)) => Ok(models::User::light_annonymus(rid.id, u)),
        _ => anyhow::bail!("timeline not found")
    }
}

pub fn mask_users_by_ids(
    user_ids: Vec<String>,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<std::collections::HashMap<std::string::String, models::User>> {

    let get_random_ids = actions::randomids::get_randomids_with_users_by_user_ids(user_ids, purpose as i32, conn)?;

    let mut user_maps = HashMap::new();
    for (rid, u) in get_random_ids {
        let mut new_user = u;
        new_user.id = rid.id;
        user_maps.insert(rid.user_id, new_user);
    }
    Ok(user_maps)
}

pub fn find_wanna_mask_userids(
    me_id: &str,
    followees: &Vec<models::User>,
    unmasked_users: Vec<models::User>,
) -> (Vec<models::User>, Vec<models::User>) {
    let mut necessary_mask_users: Vec<models::User> = Vec::new();
    let mut unnecessary_mask_users: Vec<models::User> =Vec::new();
    for uuser in unmasked_users {
        let mut is_follow = false;
        for fee in followees {
            if fee.id == uuser.id {
                is_follow = true;
            }
            if fee.id == me_id {
                is_follow =  true;
            }
        }
        if !is_follow {
            necessary_mask_users.push(uuser);
        } else {
            unnecessary_mask_users.push(uuser);
        }
    }
    (necessary_mask_users, unnecessary_mask_users)
}
