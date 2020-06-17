use scraper::{Html, Selector};
use super::super::super::models;
extern crate reqwest;
use rand::Rng;

pub async fn get_all_games(header: reqwest::header::HeaderMap) -> models::Brand {
    let client = reqwest::Client::builder()
        .default_headers(header)
        .build()
        .unwrap();

    let res = client.get("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/usersql_exec.php?sql_id=2727")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&res);
    let tr_selector = Selector::parse("tr").unwrap();
    
    let mut _brands: Vec<models::Brand> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _brand = models::Brand::get_brand_from_row(tr);
        if _brand.id != 0 {
            _brands.push(_brand);
        }
    }
    println!("{:#?}", _brands);
    _brands.get(0).unwrap().clone()
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
    println!("{:#?}", _games);
    _games.get(0).unwrap().clone()
}

impl models::Brand {
    fn get_brand_from_row(tr: scraper::element_ref::ElementRef) -> models::Brand {
        let mut _brand = models::Brand::new();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            println!("{:?}", td.inner_html());
            match i as u32 {
                0 => _brand.id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                1 => _brand.brandname = td.inner_html(),
                2 => _brand.brandfurigana = Some(td.inner_html()),
                3 => _brand.makername = Some(td.inner_html()),
                4 => _brand.makerfurigana = Some(td.inner_html()),
                5 => _brand.url = Some(td.inner_html()),
                6 => _brand.checked = option_bool_from_tf(td.inner_html()),
                7 => _brand.kind = Some(td.inner_html()),
                8 => _brand.lost = option_bool_from_tf(td.inner_html()),
                9 => _brand.directlink = option_bool_from_tf(td.inner_html()),
                10 => _brand.median = option_i32_from_string(td.inner_html()),
                11 => _brand.http_response_code = option_i32_from_string(td.inner_html()),
                12 => _brand.twitter = Some(td.inner_html()),
                13 => _brand.twitter_data_widget_id = option_i32_from_string(td.inner_html()),
                14 => _brand.notes = Some(td.inner_html()),
                15 => _brand.erogetrailers = option_i32_from_string(td.inner_html()),
                16 => _brand.cien = option_i32_from_string(td.inner_html()),
                _ => {}
            }
        }
        let today = chrono::Local::today();
        let mut rng = rand::thread_rng();
        let add_date: i64 = rng.gen_range(7, 14);
        _brand.scheduled_date = today.checked_add_signed(chrono::Duration::days(add_date)).unwrap().naive_local();
        _brand
    }
}

impl models::Game {
    fn get_game_from_row(tr: scraper::element_ref::ElementRef) -> models::Game {
        let mut _game = models::Game::new();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            println!("{:?}", td.inner_html());
            match i as u32 {
                0 => _game.id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                1 => _game.gamename = Some(td.inner_html()),
                2 => _game.furigana = Some(td.inner_html()),
                3 => _game.sellday = option_date_from_string(td.inner_html()),
                4 => _game.brand_id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                14 => _game.comike = option_i32_from_string(td.inner_html()),
                15 => _game.shoukai = Some(td.inner_html()),
                16 => _game.model = Some(td.inner_html()),
                18 => _game.erogame = option_bool_from_tf(td.inner_html()),
                21 => _game.banner_url = Some(td.inner_html()),
                26 => _game.gyutto_id = option_i32_from_string(td.inner_html()),
                27 => _game.dmm = Some(td.inner_html()),
                28 => _game.dmm_genre = Some(td.inner_html()),
                29 => _game.dmm_genre_2 = Some(td.inner_html()),
                30 => _game.erogametokuten = option_i32_from_string(td.inner_html()),
                31 => _game.total_play_time_median = option_i32_from_string(td.inner_html()),
                32 => _game.time_before_understanding_fun_median = option_i32_from_string(td.inner_html()),
                33 => _game.dlsite_id = Some(td.inner_html()),
                34 => _game.dlsite_domain = Some(td.inner_html()),
                40 => _game.trial_url = Some(td.inner_html()),
                43 => _game.okazu = option_bool_from_tf(td.inner_html()),
                44 => _game.axis_of_soft_or_hard = option_i32_from_string(td.inner_html()),
                46 => _game.genre = Some(td.inner_html()),
                47 => _game.twitter = Some(td.inner_html()),
                50 => _game.digiket = Some(td.inner_html()),
                58 => _game.twitter_data_widget_id = option_i32_from_string(td.inner_html()),
                61 => _game.masterup = option_date_from_string(td.inner_html()),
                63 => _game.steam = option_i32_from_string(td.inner_html()),
                64 => _game.dlsite_rental = option_bool_from_tf(td.inner_html()),
                65 => _game.dmm_subsc = Some(td.inner_html()),
                66 => _game.surugaya_1 = option_i32_from_string(td.inner_html()),
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

fn option_bool_from_tf(b: String) -> Option<bool> {
    match &*b {
        "t" => Some(true),
        "f" => Some(false),
        _ => None
    }
}

fn option_i32_from_string(b: String) -> Option<i32> {
    match b.parse() {
        Ok(b) => Some(b),
        _ => None
    }
}

fn option_date_from_string(b: String) -> Option<chrono::NaiveDate> {
    match b.parse() {
        Ok(b) => Some(b),
        _ => None
    }
}
