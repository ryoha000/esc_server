use serde::{Serialize};
use chrono;
use chrono::naive::NaiveDate;
use crate::schema::users;
use crate::schema::brands;
use crate::schema::games;
use crate::schema::timelines;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub es_user_id: String,
    pub display_name: String,
    pub comment: Option<String>,
    pub show_all_users: Option<bool>,
    pub show_detail_all_users: Option<bool>,
    pub show_followers: Option<bool>,
    pub show_followers_okazu: Option<bool>,
    pub twitter_id: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            id: Uuid::new_v4().to_string(),
            es_user_id: String::from(""),
            display_name: String::from(""),
            comment: None,
            show_all_users: Some(true),
            show_detail_all_users: Some(false),
            show_followers: Some(true),
            show_followers_okazu: Some(false),
            twitter_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Brand {
    pub id: i32,
    pub brandname: String,
    pub brandfurigana: Option<String>,
    pub makername: Option<String>,
    pub makerfurigana: Option<String>,
    pub url: Option<String>,
    pub checked: Option<bool>,
    pub kind: Option<String>,
    pub lost: Option<bool>,
    pub directlink: Option<bool>,
    pub median: Option<i32>,
    pub http_response_code: Option<i32>,
    pub twitter: Option<String>,
    pub twitter_data_widget_id: Option<i32>,
    pub notes: Option<String>,
    pub erogetrailers: Option<i32>,
    pub cien: Option<i32>,
    pub scheduled_date: NaiveDate,
}

impl Brand {
    pub fn new() -> Brand {
        Brand {
            id: 0,
            brandname: String::from(""),
            brandfurigana: None,
            makername: None,
            makerfurigana: None,
            url: None,
            checked: None,
            kind: None,
            lost: None,
            directlink: None,
            median: None,
            http_response_code: None,
            twitter: None,
            twitter_data_widget_id: None,
            notes: None,
            erogetrailers: None,
            cien: None,
            scheduled_date: NaiveDate::from_ymd(2030, 3, 31),
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable)]
#[belongs_to(Brand)]
pub struct Game {
    pub id: i32,
    pub gamename: Option<String>,
    pub furigana: Option<String>,
    pub sellday: Option<NaiveDate>,
    pub brand_id: i32,
    // pub median: Option<i32>,
    // pub stdev: Option<i32>,
    // pub count2: Option<i32>,
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
    pub masterup: Option<NaiveDate>,
    // pub masterup_tourokubi: Option<chrono::NaiveDateTime>,
    pub steam: Option<i32>,
    pub dlsite_rental: Option<bool>,
    pub dmm_subsc: Option<String>,
    pub surugaya_1: Option<i32>,
    pub scheduled_date: NaiveDate,
}

impl Game {
    pub fn new() -> Game {
        Game {
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
            scheduled_date: NaiveDate::from_ymd(2030, 3, 31),
        }
    }
}

pub struct Timeline {
    pub id: String,
    pub user_id: String,
    pub game_id: i32,
    pub log_type: i32, // Play => 0, Review => 1, List = 2
    pub created_at: chrono::NaiveDateTime,
}

impl Timeline {
    pub fn new(user_id: String, game_id: i32, log_type: i32) -> Timeline {
        Timeline {
            id: Uuid::new_v4().to_string(),
            user_id: user_id,
            game_id: game_id,
            log_type: log_type,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)
        }
    }
}