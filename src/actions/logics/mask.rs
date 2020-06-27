use super::super::super::models;
use super::super::super::middleware;
use super::super::super::api::timelines;
use super::super::super::RedisPool;
use super::super::super::actions;

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
    auth: middleware::Authorized,
    timeline_id: String,
    redis_pool: RedisPool,
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

    // Requestしたユーザーを取得
    let mut redis_conn = redis_pool.get().context("couldn't get db connection from pools")?;
    let me = middleware::check_user(auth, &mut redis_conn);

    // 匿名化する場合のidを取得
    let random_id = actions::randomids::get_randomid_by_user_id(res_tl.user_id.clone(), models::RandomPurpose::FTimeline as i32, conn)?;

    match me {
        Some(_me) => {
            let followees = actions::follows::find_followees_by_uid(_me.user_id, conn)?;

            // action主のfollowerかどうか確認
            let mut is_follow = false;
            for flee in &followees {
                if res_tl.user_id.clone() == flee.id {
                    is_follow = true;
                }
            }

            let mut user = models::User::annonymus(random_id.id.clone(), String::from(""));
            match is_follow {
                true => {
                    if let Some(getted_user) = actions::users::find_user_by_uid(res_tl.user_id.clone(), conn)? {
                        user = getted_user;
                    }
                },
                false => {
                    res_tl.user_id = random_id.id;
                }
            }

            // reviewかListを挿入
            let mut _review: Option<models::Review> = None;
            let mut _list: Option<models::List> = None;
            match res_tl.log_type.clone() {
                // Play => 0, Review => 1, List = 2
                1 => {
                    if let Some((_reviewlog, found_review)) = actions::reviewlogs::find_review_by_timeline_id(timeline_id.clone(), conn)? {
                        _review = Some(found_review);
                    }
                },
                2 => {
                    if let Some((_listlog, found_list)) = actions::listlogs::find_list_by_timeline_id(timeline_id, conn)? {
                        _list = Some(found_list);
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
        },
        _ => {
            anyhow::bail!("somethin went wrong")
        }
    }
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
    let get_random_ids = actions::randomids::get_randomids_by_user_ids(user_ids, purpose as i32, conn)?;

    println!("{:?}", get_random_ids);
    Ok(get_random_ids)
}
