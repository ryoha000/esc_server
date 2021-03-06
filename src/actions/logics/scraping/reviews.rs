use scraper::{Selector};
use super::super::super::super::models;
extern crate reqwest;
use super::scraping_shared::*;
use anyhow::{Result};

const INVALID_GAME_ID: [i32; 10] = [23707, 4370, 16061, 4371, 4372, 4373, 29250, 15353, 26836, 9381];

pub async fn get_all_reviews() -> Result<Vec<models::Review>> {
    let mut _reviews: Vec<models::Review> = Vec::new();
    let get_review = async {
        for i in 0..120 {
            let query = format!("WHERE id > {} AND id < {}", i * 20000, i * 20000 + 20000);
            println!("{}", query);
            let fragment = execute_on_es(make_query(query)).await.unwrap();
            let tr_selector = Selector::parse("tr").unwrap();
            for tr in fragment.select(&tr_selector) {
                let mut _review = models::Review::get_review_from_row(tr);
                if check_game(&_review.game_id) {
                    _reviews.push(_review);
                }
            }
        }
    };
    get_review.await;
    Ok(_reviews)
}

pub async fn get_recent_reviews() -> Result<Vec<models::Review>> {
    let mut _reviews: Vec<models::Review> = Vec::new();
    let query = format!("WHERE CURRENT_TIMESTAMP - interval '5 minute' < modified");
    let fragment = execute_on_es(make_query(query)).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    for tr in fragment.select(&tr_selector) {
        let mut _review = models::Review::get_review_from_row(tr);
        if check_game(&_review.game_id) {
            _reviews.push(_review);
        }
    }
    Ok(_reviews)
}

pub async fn get_reviews_by_es_user_id(user_id: String) -> Result<Vec<models::Review>> {
    let mut _reviews: Vec<models::Review> = Vec::new();
    let query = format!("WHERE uid = '{}'", user_id);
    let fragment = execute_on_es(make_query(query)).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    for tr in fragment.select(&tr_selector) {
        let mut _review = models::Review::get_review_from_row(tr);
        if check_game(&_review.game_id) {
            _reviews.push(_review);
        }
    }
    Ok(_reviews)
}

fn make_query(where_query: String) -> String {
    format!("{}{}", r"
SELECT
    game,
    uid,
    tokuten,
    tourokubi,
    hitokoto,
    memo,
    netabare,
    giveup,
    possession,
    play,
    before_hitokoto,
    before_tokuten,
    before_tourokubi,
    display,
    play_tourokubi,
    display_unique_count,
    sage,
    before_purchase_will,
    before_sage,
    total_play_time,
    time_before_understanding_fun,
    okazu_tokuten,
    trial_version_hitokoto,
    trial_version_hitokoto_sage,
    trial_version_hitokoto_tourokubi,
    timestamp,
    id
FROM userreview
        " , where_query)
}

fn check_game(game_id: &i32) -> bool {
    let mut is_ok = true;
    if game_id == &0 {
        is_ok = false;
    }
    for inv_id in INVALID_GAME_ID.iter() {
        if game_id == inv_id {
            is_ok = false;
        }
    }
    is_ok
}

impl models::Review {
    fn get_review_from_row(tr: scraper::element_ref::ElementRef) -> models::Review {
        let mut _review = models::Review::initialize();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            match (i + 1) as u32 {
                1 => _review.game_id = i32_from_string(td.inner_html()),
                2 => _review.es_user_id = td.inner_html(),
                3 => _review.tokuten = option_i32_from_string(td.inner_html()),
                4 => _review.tourokubi = option_datetime_from_string(td.inner_html()),
                5 => _review.hitokoto = option_string_from_string(td.inner_html()),
                6 => _review.memo = option_string_from_string(td.inner_html()),
                7 => _review.netabare = option_bool_from_tf(td.inner_html()),
                8 => _review.giveup = option_bool_from_tf(td.inner_html()),
                9 => _review.possession = option_bool_from_tf(td.inner_html()),
                10 => _review.play = option_bool_from_tf(td.inner_html()),
                11 => _review.before_hitokoto = option_string_from_string(td.inner_html()),
                12 => _review.before_tokuten = option_i32_from_string(td.inner_html()),
                13 => _review.before_tourokubi = option_datetime_from_string(td.inner_html()),
                14 => _review.display = option_bool_from_tf(td.inner_html()),
                15 => _review.play_tourokubi = option_datetime_from_string(td.inner_html()),
                16 => _review.display_unique_count = option_i32_from_string(td.inner_html()),
                17 => _review.sage = option_bool_from_tf(td.inner_html()),
                18 => _review.before_purchase_will = option_string_from_string(td.inner_html()),
                19 => _review.before_sage = option_bool_from_tf(td.inner_html()),
                20 => _review.total_play_time = option_i32_from_string(td.inner_html()),
                21 => _review.time_before_understanding_fun = option_i32_from_string(td.inner_html()),
                22 => _review.okazu_tokuten = option_i32_from_string(td.inner_html()),
                23 => _review.trial_version_hitokoto = option_string_from_string(td.inner_html()),
                24 => _review.trial_version_hitokoto_sage = option_bool_from_tf(td.inner_html()),
                25 => _review.trial_version_hitokoto_tourokubi = option_datetime_from_string(td.inner_html()),
                26 => _review.created_at = option_datetime_from_string(td.inner_html()),
                27 => _review.es_id = option_i32_from_string(td.inner_html()),
                _ => {}
            }
        }
        _review
    }
}
