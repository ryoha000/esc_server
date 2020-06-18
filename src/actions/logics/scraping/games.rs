use scraper::{Html, Selector};
use super::super::super::super::models;
extern crate reqwest;
use rand::Rng;
use super::scraping_shared::*;
use anyhow::{Context, Result};

pub async fn get_latest_brand_by_id(id: i32) -> Result<models::Game> {
    let id = format!("'{}'", id);
    let fragment = execute_on_es(format!("{}{}", r"
            SELECT 
                id,
                gamename ,
                furigana ,
                sellday,
                brandname ,
                comike	,
                shoukai	,
                model	,
                erogame	,
                banner_url	,
                gyutto_id	,
                dmm	,
                dmm_genre	,
                dmm_genre_2	,
                erogametokuten ,
                total_play_time_median ,	
                time_before_understanding_fun_median ,	
                dlsite_id	,
                dlsite_domain	,
                trial_url	,
                okazu	,
                axis_of_soft_or_hard ,
                genre	,
                twitter	,
                digiket	,
                twitter_data_widget_id ,
                masterup ,
                steam ,
                dlsite_rental ,
                dmm_subsc ,
                surugaya_1
            FROM gamelist
            WHERE id = 
        " , id)).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut select_tr = fragment.select(&tr_selector);
    select_tr.next().with_context(|| "there is no tr")?;
    let tr = select_tr.next().with_context(|| "Not Found")?;
    let mut _game = models::Game::get_game_from_row(tr);
    Ok(_game)
}

pub async fn get_latest_game_by_id(id: i32) -> Result<models::Game> {
    let id = format!("'{}'", id);
    let fragment = execute_on_es(format!("{}{}", r"
            SELECT 
                id,
                gamename ,
                furigana ,
                sellday,
                brandname ,
                comike	,
                shoukai	,
                model	,
                erogame	,
                banner_url	,
                gyutto_id	,
                dmm	,
                dmm_genre	,
                dmm_genre_2	,
                erogametokuten ,
                total_play_time_median ,	
                time_before_understanding_fun_median ,	
                dlsite_id	,
                dlsite_domain	,
                trial_url	,
                okazu	,
                axis_of_soft_or_hard ,
                genre	,
                twitter	,
                digiket	,
                twitter_data_widget_id ,
                masterup ,
                steam ,
                dlsite_rental ,
                dmm_subsc ,
                surugaya_1
            FROM gamelist
            WHERE id = 
        " , id)).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut select_tr = fragment.select(&tr_selector);
    select_tr.next().with_context(|| "there is no tr")?;
    let tr = select_tr.next().with_context(|| "Not Found")?;
    let mut _game = models::Game::get_game_from_row(tr);
    Ok(_game)
}

pub async fn get_test_game(header: reqwest::header::HeaderMap) -> models::Game {
    let client = reqwest::Client::builder()
        .default_headers(header)
        .build()
        .unwrap();

    let res = client.get("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/usersql_exec.php?sql_id=2726")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&res);
    let tr_selector = Selector::parse("tr").unwrap();
    
    let mut _games: Vec<models::Game> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _game = models::Game::get_game_from_row(tr);
        if _game.id != 0 {
            _games.push(_game);
        }
    }
    _games.get(0).unwrap().clone()
}

impl models::Game {
    fn get_game_from_row(tr: scraper::element_ref::ElementRef) -> models::Game {
        let mut _game = models::Game::new();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            match i as u32 {
                0 => _game.id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                1 => _game.gamename = option_string_from_string(td.inner_html()),
                2 => _game.furigana = option_string_from_string(td.inner_html()),
                3 => _game.sellday = option_date_from_string(td.inner_html()),
                4 => _game.brand_id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                5 => _game.comike = option_i32_from_string(td.inner_html()),
                6 => _game.shoukai = option_string_from_string(td.inner_html()),
                7 => _game.model = option_string_from_string(td.inner_html()),
                8 => _game.erogame = option_bool_from_tf(td.inner_html()),
                9 => _game.banner_url = option_string_from_string(td.inner_html()),
                10 => _game.gyutto_id = option_i32_from_string(td.inner_html()),
                11 => _game.dmm = option_string_from_string(td.inner_html()),
                12 => _game.dmm_genre = option_string_from_string(td.inner_html()),
                13 => _game.dmm_genre_2 = option_string_from_string(td.inner_html()),
                14 => _game.erogametokuten = option_i32_from_string(td.inner_html()),
                15 => _game.total_play_time_median = option_i32_from_string(td.inner_html()),
                16 => _game.time_before_understanding_fun_median = option_i32_from_string(td.inner_html()),
                17 => _game.dlsite_id = option_string_from_string(td.inner_html()),
                18 => _game.dlsite_domain = option_string_from_string(td.inner_html()),
                19 => _game.trial_url = option_string_from_string(td.inner_html()),
                20 => _game.okazu = option_bool_from_tf(td.inner_html()),
                21 => _game.axis_of_soft_or_hard = option_i32_from_string(td.inner_html()),
                22 => _game.genre = option_string_from_string(td.inner_html()),
                23 => _game.twitter = option_string_from_string(td.inner_html()),
                24 => _game.digiket = option_string_from_string(td.inner_html()),
                25 => _game.twitter_data_widget_id = option_i32_from_string(td.inner_html()),
                26 => _game.masterup = option_date_from_string(td.inner_html()),
                27 => _game.steam = option_i32_from_string(td.inner_html()),
                28 => _game.dlsite_rental = option_bool_from_tf(td.inner_html()),
                29 => _game.dmm_subsc = option_string_from_string(td.inner_html()),
                30 => _game.surugaya_1 = option_i32_from_string(td.inner_html()),
                _ => {}
            }
        }
        let today = chrono::Local::today();
        let mut rng = rand::thread_rng();
        let add_date: i64 = rng.gen_range(7, 14);
        _game.scheduled_date = today.checked_add_signed(chrono::Duration::days(add_date)).unwrap().naive_local();
        _game
    }
}
