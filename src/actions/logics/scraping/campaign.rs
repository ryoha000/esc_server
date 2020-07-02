use scraper::{Selector};
extern crate reqwest;
use super::scraping_shared::*;
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use chrono;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub end_timestamp: Option<chrono::NaiveDateTime>,
    pub content: Option<String>,
    pub store: Option<i32>,
    pub games: Vec<CampaignGame>,
}

impl Campaign {
    fn new() -> Campaign {
        Campaign {
            id: 0,
            name: None,
            url: None,
            end_timestamp: None,
            content: None,
            store: None,
            games: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignGame {
    pub id: i32,
    pub gamename: Option<String>,
    pub furigana: Option<String>,
    pub sellday: Option<chrono::NaiveDate>,
    pub brand_id: i32,
    pub comike: Option<i32>,
    pub shoukai: Option<String>,
    pub model: Option<String>,
    pub erogame: Option<bool>,
    pub banner_url: Option<String>,
    pub gyutto_id: Option<i32>,
    pub dmm: Option<String>,
    pub dmm_genre: Option<String>,
    pub dmm_genre_2: Option<String>,
    pub erogametokuten: Option<i32>,
    pub total_play_time_median: Option<i32>,
    pub time_before_understanding_fun_median: Option<i32>,
    pub dlsite_id: Option<String>,
    pub dlsite_domain: Option<String>,
    pub trial_url: Option<String>,
    pub okazu: Option<bool>,
    pub axis_of_soft_or_hard: Option<i32>,
    pub genre: Option<String>,
    pub twitter: Option<String>,
    pub digiket: Option<String>,
    pub twitter_data_widget_id: Option<i32>,
    pub masterup: Option<chrono::NaiveDate>,
    pub steam: Option<i32>,
    pub dlsite_rental: Option<bool>,
    pub dmm_subsc: Option<String>,
    pub surugaya_1: Option<i32>,
    pub median: i32,
    pub stdev: i32,
    pub count2: i32,
    pub content: Option<String>,
    pub url: Option<String>,
}

impl CampaignGame {
    fn new() -> CampaignGame {
        CampaignGame {
            id: 0,
            gamename: None,
            furigana: None,
            sellday: None,
            brand_id: 0,
            comike: None,
            shoukai: None,
            model: None,
            erogame: None,
            banner_url: None,
            gyutto_id: None,
            dmm: None,
            dmm_genre: None,
            dmm_genre_2: None,
            erogametokuten: None,
            total_play_time_median: None,
            time_before_understanding_fun_median: None,
            dlsite_id: None,
            dlsite_domain: None,
            trial_url: None,
            okazu: None,
            axis_of_soft_or_hard: None,
            genre: None,
            twitter: None,
            digiket: None,
            twitter_data_widget_id: None,
            masterup: None,
            steam: None,
            dlsite_rental: None,
            dmm_subsc: None,
            surugaya_1: None,
            median: 0,
            stdev: 0,
            count2: 0,
            content: None,
            url: None,
        }
    }
}

pub async fn get_now_campaign() -> Result<HashMap<i32, Campaign>> {
    let fragment = execute_on_es(make_query()).await.unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let mut campaigns: HashMap<i32, Campaign> = HashMap::new();
    for tr in fragment.select(&tr_selector) {
        let (_campaign, _game) = Campaign::get_campaign_from_row(tr);
        if _game.id == 0 && _campaign.id == 0 {
            continue
        }
        let inserted_campaign = campaigns.entry(_campaign.id).or_insert(_campaign);
        (*inserted_campaign).games.push(_game);
    }
    Ok(campaigns)
}

fn make_query() -> String {
    format!("{}", r"
            SELECT 
                gamelist.id,
                gamelist.gamename ,
                gamelist.furigana ,
                gamelist.sellday,
                gamelist.brandname ,
                gamelist.comike	,
                gamelist.shoukai	,
                gamelist.model	,
                gamelist.erogame	,
                gamelist.banner_url	,
                gamelist.gyutto_id	,
                gamelist.dmm	,
                gamelist.dmm_genre	,
                gamelist.dmm_genre_2	,
                gamelist.erogametokuten ,
                gamelist.total_play_time_median ,	
                gamelist.time_before_understanding_fun_median ,	
                gamelist.dlsite_id	,
                gamelist.dlsite_domain	,
                gamelist.trial_url	,
                gamelist.okazu	,
                gamelist.axis_of_soft_or_hard ,
                gamelist.genre	,
                gamelist.twitter	,
                gamelist.digiket	,
                gamelist.twitter_data_widget_id ,
                gamelist.masterup ,
                gamelist.steam ,
                gamelist.dlsite_rental ,
                gamelist.dmm_subsc ,
                gamelist.surugaya_1,
                gamelist.median,
                gamelist.stdev,
                gamelist.count2,
                cg.content,
                cg.url,
                cl.store,
                cl.id,
                cl.name,
                cl.url,
                cl.content,
                cl.end_timestamp
            FROM campaign_game cg, campaignlist cl, gamelist
            WHERE cg.campaign = cl.id AND index_end_timestamp>=CURRENT_TIMESTAMP AND gamelist.id = cg.game
        ")
}

impl Campaign {
    fn get_campaign_from_row(tr: scraper::element_ref::ElementRef) -> (Campaign, CampaignGame) {
        let mut _campaign = Campaign::new();
        let mut _game = CampaignGame::new();
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
                31 => _game.median = i32_from_string(td.inner_html()),
                32 => _game.stdev = i32_from_string(td.inner_html()),
                33 => _game.count2 = i32_from_string(td.inner_html()),
                34 => _game.content = option_string_from_string(td.inner_html()),
                35 => _game.url = option_string_from_string(td.inner_html()),
                36 => _campaign.store = option_i32_from_string(td.inner_html()),
                37 => _campaign.id = i32_from_string(td.inner_html()),
                38 => _campaign.name = option_string_from_string(td.inner_html()),
                39 => _campaign.url = option_string_from_string(td.inner_html()),
                40 => _campaign.content = option_string_from_string(td.inner_html()),
                41 => _campaign.end_timestamp = option_datetime_from_string(td.inner_html()),
                _ => {}
            }
        }
        (_campaign, _game)
    }
}
