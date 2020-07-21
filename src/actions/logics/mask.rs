use super::super::super::models;
use super::super::super::middleware;
use super::super::super::api::timelines;
use super::super::super::actions;
use std::collections::HashMap;


use diesel::prelude::*;
use anyhow::{Context, Result};

pub fn mask_recent_timelines(
    me: Option<middleware::Me>,
    offset: i64,
    conn: &PgConnection,
) -> Result<Vec<timelines::MaskedTimeline>> {
    // 自分がフォローしてるユーザーのIDの配列
    let mut followee_user_ids: Vec<String> = Vec::new();
    // 自分がフォローしてるユーザーの配列
    let mut followee_users: Vec<models::User> = Vec::new();
    match me {
        Some(_me) => {
            let my_uuid: uuid::Uuid = _me.user_id.parse().context("please enter uuid")?;
            // 自分の行動もマスクする必要ない
            followee_user_ids.push(_me.user_id.clone());

            if let Some(followees) = actions::follows::find_followees_by_uid(my_uuid, conn)? {
                // 自分がフォローしてるユーザーを入れていく
                for flee in &followees {
                    followee_user_ids.push(flee.id.clone());
                }
                followee_users = followees;
            }

            if let Some(user) = actions::users::find_user_by_uid(_me.user_id, conn)? {
                // 自分もいれる
                followee_users.push(user);
            }
        }
        _ => {}
    }

    // timelineとかの配列
    let tl_with_game_vec: Vec<(models::Timeline, models::Game)>;
    // maskする必要があるuseridの配列
    let mut necessary_mask_user_ids: Vec<String> = Vec::new();
    if let Some(_tl_with_game_vec) = actions::timelines::find_timelines_with_game_of_limit20_by_unnecessary_mask_user_ids(offset, conn)? {
        tl_with_game_vec = _tl_with_game_vec;
        for (tl, _) in &tl_with_game_vec {
            let mut is_follow = false;
            for uid in &followee_user_ids {
                if uid == &tl.user_id {
                    is_follow = true;
                }
            }
            if !is_follow {
                necessary_mask_user_ids.push(tl.user_id.clone());
            }
        }
    } else {
        anyhow::bail!("timeline not found")
    }

    // 匿名化したuserのハッシュマップを用意
    let masked_users_map = mask_annynomus_users_by_ids(necessary_mask_user_ids, models::RandomPurpose::FTimeline, conn)?;

    // 返却する配列
    let mut res_timelines: Vec<timelines::MaskedTimeline> = Vec::new();
    for (tl, gm) in tl_with_game_vec {
        let mut res_tl = tl;
        let mut res_user: models::User = models::User::new();
        match masked_users_map.get(&res_tl.user_id) {
            Some(masked_user) => {
                if masked_user.show_all_users == Some(false) {
                    continue
                }
                res_tl.user_id = masked_user.id.clone();
                res_user = masked_user.clone();
            },
            _ => {
                let mut is_error = true;
                for flee in &followee_users {
                    if flee.id == res_tl.user_id {
                        if flee.show_followers == Some(false) || (gm.okazu == Some(true) && flee.show_followers_okazu == Some(false)) {
                            continue
                        }
                        res_user = flee.clone();
                        is_error = false;
                        break
                    }
                }
                if is_error {
                    anyhow::bail!("something went wrong")
                }
                res_user.password = String::from("");
            }
        }
        res_timelines.push(
            timelines::MaskedTimeline {
                timeline: res_tl,
                list: None,
                review: None,
                game: gm,
                user: res_user,
            }
        )
    }

    Ok(res_timelines)
}

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

    let mut user: models::User;
    let mut is_follow: bool = false;
    let mut rid_id = String::from("");
    if res_tl.user_id.clone() != String::from("") {
        // 匿名化する場合のidを取得
        let random_id = actions::randomids::get_randomid_by_user_id(res_tl.user_id.clone(), models::RandomPurpose::FTimeline as i32, conn)?;
        rid_id = random_id.id.clone();
    
        match me {
            Some(_me) => {
                let my_uuid: uuid::Uuid = _me.user_id.parse().context("please enter uuid")?;
    
                if let Some(followees) = actions::follows::find_followees_by_uid(my_uuid, conn)? {
                    // action主のfollowerかどうか確認
                    for flee in &followees {
                        if res_tl.user_id == flee.id {
                            is_follow = true;
                        }
                    }
                }
                if res_tl.user_id == _me.user_id {
                    is_follow = true;
                }
            }
            _ => {}
        }
    
        if res_tl.log_type == 2 && !is_follow {
            anyhow::bail!("this is unfollow user list activity")
        }

        if let Some(getted_user) = actions::users::find_user_by_uid(res_tl.user_id.clone(), conn)? {
            match is_follow {
                true => {
                    if getted_user.show_followers == Some(false) || (res_game.okazu == Some(true) && getted_user.show_followers_okazu == Some(false)) {
                        anyhow::bail!("this user not show activity")
                    }
                    user = getted_user;
                    user.password = String::from("");
                },
                false => {
                    if getted_user.show_all_users == Some(false) {
                        anyhow::bail!("this user not show activity")
                    }
                    user = models::User::annonymus(random_id.id.clone(), String::from(""), String::from("名無しさん"));
                    res_tl.user_id = random_id.id.clone();
                }
            }
        } else {
            user = models::User::annonymus(random_id.id.clone(), String::from(""), String::from("名無しさん"));
        }
    } else {
        if res_tl.log_type == models::LogType::Review as i32 {
            user = models::User::annonymus(res_tl.user_id.clone(), String::from("批評空間のユーザー"), String::from("批評空間のユーザー"));
        } else {
            user = models::User::annonymus(res_tl.user_id.clone(), String::from(""), String::from("名無しさん"));
        }
    }

    // reviewかListを挿入
    let mut _review: Option<models::Review> = None;
    let mut _list: Option<models::List> = None;
    match res_tl.log_type.clone() {
        // Play => 0, Review => 1, List = 2
        1 => {
            if let Some((_reviewlog, found_review)) = actions::reviewlogs::find_review_by_timeline_id(timeline_id.clone(), conn)? {
                println!("{:?}", found_review);
                let mut assign_review = found_review;
                if !is_follow {
                    assign_review.user_id = rid_id;
                }
                _review = Some(assign_review);
            }
        },
        2 => {
            if let Some((_listlog, found_list)) = actions::listlogs::find_list_by_timeline_id(timeline_id, conn)? {
                let mut assign_list = found_list;
                if !is_follow {
                    assign_list.user_id = rid_id;
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
        _ => anyhow::bail!("masked user not found")
    }
}

pub fn mask_annynomus_users_by_ids(
    user_ids: Vec<String>,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<std::collections::HashMap<std::string::String, models::User>> {

    let get_random_ids = actions::randomids::get_randomids_with_users_by_user_ids(user_ids, purpose as i32, conn)?;

    let mut user_maps = HashMap::new();
    for (rid, u) in get_random_ids {
        let masked_es_id: String;
        let masked_display_name: String;
        match &*u.display_name {
            "批評空間のユーザー" => {
                masked_es_id = String::from("批評空間のユーザー");
                masked_display_name = String::from("批評空間のユーザー");
            },
            _ => {
                masked_es_id = String::from("");
                masked_display_name = String::from("名無しさん");
            }
        }
        user_maps.insert(rid.user_id, models::User::annonymus(rid.id, masked_es_id, masked_display_name));
    }
    Ok(user_maps)
}

pub fn mask_users_by_ids(
    user_ids: Vec<String>,
    purpose: models::RandomPurpose,
    conn: &PgConnection,
) -> Result<std::collections::HashMap<std::string::String, models::User>> {

    let get_random_ids = actions::randomids::get_randomids_with_users_by_user_ids(user_ids, purpose as i32, conn)?;

    let mut user_maps = HashMap::new();
    for (rid, u) in get_random_ids {
        user_maps.insert(rid.user_id, models::User::light_annonymus(rid.id, u));
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
