use scraper::{Selector};
use super::super::super::super::models;
extern crate reqwest;
use rand::Rng;
use super::scraping_shared::*;
use anyhow::{Context, Result};

pub async fn get_all_brands() -> Result<Vec<models::Brand>> {
    let fragment = execute_on_es(make_query(format!(""))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let mut _brands: Vec<models::Brand> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _brand = models::Brand::get_brand_from_row(tr);
        if _brand.id != 0 {
            _brands.push(_brand);
        }
    }
    Ok(_brands)
}

pub async fn get_latest_brand_by_id(id: i32) -> Result<models::Brand> {
    let fragment = execute_on_es(make_query(format!("WHERE id = '{}'", id))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut select_tr = fragment.select(&tr_selector);
    select_tr.next().with_context(|| "there is no tr")?;
    let tr = select_tr.next().with_context(|| "Not Found")?;
    let mut _brand = models::Brand::get_brand_from_row(tr);
    Ok(_brand)
}

pub async fn get_latest_brands_by_id(id: i32) -> Result<Vec<models::Brand>> {
    let fragment = execute_on_es(make_query(format!("WHERE id > '{}'", id))).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let mut _brands: Vec<models::Brand> = Vec::new();
    for tr in fragment.select(&tr_selector) {
        let mut _brand = models::Brand::get_brand_from_row(tr);
        if _brand.id != 0 {
            _brands.push(_brand);
        }
    }
    Ok(_brands)
}

fn make_query(query_where: String) -> String {
    format!("{}{}", r"
            SELECT 
                id,
                brandname,
                brandfurigana,
                makername,
                makerfurigana,
                url,
                checked,
                kind,
                lost,
                directlink,
                median,
                http_response_code,
                twitter,
                twitter_data_widget_id,
                notes,
                erogetrailers,
                cien
            FROM brandlist 
        " , query_where)
}

impl models::Brand {
    fn get_brand_from_row(tr: scraper::element_ref::ElementRef) -> models::Brand {
        let mut _brand = models::Brand::new();
        let td_selector = Selector::parse("td").unwrap();
    
        for (i, td) in tr.select(&td_selector).enumerate() {
            match i as u32 {
                0 => _brand.id = match td.inner_html().parse() {
                    Ok(b) => b,
                    _ => 0
                },
                1 => _brand.brandname = td.inner_html(),
                2 => _brand.brandfurigana = option_string_from_string(td.inner_html()),
                3 => _brand.makername = option_string_from_string(td.inner_html()),
                4 => _brand.makerfurigana = option_string_from_string(td.inner_html()),
                5 => _brand.url = option_string_from_string(td.inner_html()),
                6 => _brand.checked = option_bool_from_tf(td.inner_html()),
                7 => _brand.kind = option_string_from_string(td.inner_html()),
                8 => _brand.lost = option_bool_from_tf(td.inner_html()),
                9 => _brand.directlink = option_bool_from_tf(td.inner_html()),
                10 => _brand.median = option_i32_from_string(td.inner_html()),
                11 => _brand.http_response_code = option_i32_from_string(td.inner_html()),
                12 => _brand.twitter = option_string_from_string(td.inner_html()),
                13 => _brand.twitter_data_widget_id = option_i32_from_string(td.inner_html()),
                14 => _brand.notes = option_string_from_string(td.inner_html()),
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