use serde::{Serialize};
use chrono;
use crate::schema::users;
use crate::schema::brands;

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
    pub name: String,
    pub display_name: String,
    pub password: String,
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
    pub scheduled_date: chrono::naive::NaiveDate,
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
            scheduled_date: chrono::naive::NaiveDate::from_ymd(2030, 3, 31),
        }
    }
}
