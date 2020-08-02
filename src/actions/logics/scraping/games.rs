use scraper::{Html, Selector};
use super::super::super::super::models;
extern crate reqwest;
use super::scraping_shared::*;
use anyhow::{Context, Result};

const INVALID_GAME_ID: [i32; 10] = [23707, 4370, 16061, 4371, 4372, 4373, 29250, 15353, 26836, 9381];

pub async fn get_all_games() -> Result<Vec<models::Game>> {
    let fragment = execute_on_es(make_query(format!(""))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let mut _games: Vec<models::Game> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _game = models::Game::get_game_from_row(tr);
        if check_game(&_game) {
            _games.push(_game);
        }
    }
    Ok(_games)
}

pub async fn get_latest_game_by_id(id: i32) -> Result<models::Game> {
    let fragment = execute_on_es(make_query(format!("where id = '{}'", id))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut select_tr = fragment.select(&tr_selector);
    select_tr.next().with_context(|| "there is no tr")?;
    let tr = select_tr.next().with_context(|| "Not Found")?;
    let mut _game = models::Game::get_game_from_row(tr);
    Ok(_game)
}

pub async fn get_latest_games_by_id(id: i32) -> Result<Vec<models::Game>> {
    let fragment = execute_on_es(make_query(format!("where id > '{}'", id))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut _games: Vec<models::Game> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _game = models::Game::get_game_from_row(tr);
        if check_game(&_game) {
            _games.push(_game);
        }
    }
    Ok(_games)
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
        if check_game(&_game) {
            _games.push(_game);
        }
    }
    _games.get(0).unwrap().clone()
}

fn make_query(query_where: String) -> String {
    format!("{}{}", r"
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
                genre	,
                twitter	,
                twitter_data_widget_id ,
                masterup ,
                steam ,
                dlsite_rental ,
                dmm_subsc ,
                surugaya_1,
                median,
                stdev,
                count2
            FROM gamelist 
        " , query_where)
}

fn check_game(_game: &models::Game) -> bool {
    let mut is_ok = true;
    if _game.id == 0 || _game.brand_id == 0 {
        is_ok = false;
    }
    for inv_id in INVALID_GAME_ID.iter() {
        if &_game.id == inv_id {
            is_ok = false;
        }
    }
    is_ok
}

impl models::Game {
    fn get_game_from_row(tr: scraper::element_ref::ElementRef) -> models::Game {
        let mut _game = models::Game::new();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            match i as u32 {
                0 => _game.id = i32_from_string(td.inner_html()),
                1 => _game.gamename = option_string_from_string(td.inner_html()),
                2 => _game.furigana = option_string_from_string(td.inner_html()),
                3 => _game.sellday = option_date_from_string(td.inner_html()),
                4 => _game.brand_id = i32_from_string(td.inner_html()),
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
                //21 => _game.axis_of_soft_or_hard = option_i32_from_string(td.inner_html()),
                21 => _game.genre = option_string_from_string(td.inner_html()),
                22 => _game.twitter = option_string_from_string(td.inner_html()),
                //24 => _game.digiket = option_string_from_string(td.inner_html()),
                23 => _game.twitter_data_widget_id = option_i32_from_string(td.inner_html()),
                24 => _game.masterup = option_date_from_string(td.inner_html()),
                25 => _game.steam = option_i32_from_string(td.inner_html()),
                26 => _game.dlsite_rental = option_bool_from_tf(td.inner_html()),
                27 => _game.dmm_subsc = option_string_from_string(td.inner_html()),
                28 => _game.surugaya_1 = option_i32_from_string(td.inner_html()),
                29 => _game.median = option_i32_from_string(td.inner_html()),
                30 => _game.stdev = option_i32_from_string(td.inner_html()),
                31 => _game.count2 = option_i32_from_string(td.inner_html()),
                _ => {}
            }
        }
        _game
    }
}
