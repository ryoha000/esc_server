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
        let mut _brand = get_brand_from_row(tr);
        if _brand.id != 0 {
            _brands.push(_brand);
        }
    }
    println!("{:#?}", _brands);
    _brands.get(0).unwrap().clone()
}

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