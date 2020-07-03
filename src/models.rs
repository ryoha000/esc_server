use serde::{Serialize};
use chrono;
use chrono::naive::NaiveDate;
use crate::schema::users;
use crate::schema::brands;
use crate::schema::games;
use crate::schema::timelines;
use crate::schema::follows;
use crate::schema::lists;
use crate::schema::listmaps;
use crate::schema::listlogs;
use crate::schema::reviews;
use crate::schema::reviewlogs;
use crate::schema::messages;
use crate::schema::randomids;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable, QueryableByName)]
#[table_name = "users"]
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
    pub icon_url: Option<String>,
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
            icon_url: None,
        }
    }
    pub fn annonymus(randomid: String, es_user_id: String) -> User {
        User {
            id: randomid,
            es_user_id: es_user_id,
            display_name: String::from("名無しさん"),
            comment: None,
            show_all_users: Some(true),
            show_detail_all_users: Some(false),
            show_followers: Some(true),
            show_followers_okazu: Some(false),
            twitter_id: None,
            icon_url: None,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "games"]
#[belongs_to(Brand)]
pub struct Game {
    pub id: i32,
    pub gamename: Option<String>,
    pub furigana: Option<String>,
    pub sellday: Option<NaiveDate>,
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
    pub genre: Option<String>,
    pub twitter: Option<String>,
    pub twitter_data_widget_id: Option<i32>,
    pub masterup: Option<NaiveDate>,
    // pub masterup_tourokubi: Option<chrono::NaiveDateTime>,
    pub steam: Option<i32>,
    pub dlsite_rental: Option<bool>,
    pub dmm_subsc: Option<String>,
    pub surugaya_1: Option<i32>,
    pub median: Option<i32>,
    pub stdev: Option<i32>,
    pub count2: Option<i32>,
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
            genre: None,
            twitter: None,
            twitter_data_widget_id: None,
            masterup: None,
            steam: None,
            dlsite_rental: None,
            dmm_subsc: None,
            surugaya_1: None,
            median: None,
            stdev: None,
            count2: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable)]
#[belongs_to(User)]
#[belongs_to(Game)]
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

pub enum LogType {
    Play,
    Review,
    List,
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[belongs_to(parent = User, foreign_key = "followee_id")]
#[belongs_to(parent = User, foreign_key = "follower_id")]
#[table_name = "follows"]
pub struct Follow {
    pub id: String,
    pub followee_id: String,
    pub follower_id: String,
    pub allowed: bool,
    pub mutual: bool,
    pub comment: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl Follow {
    pub fn new(followee_id: String, follower_id: String) -> Follow {
        Follow {
            id: Uuid::new_v4().to_string(),
            followee_id: followee_id,
            follower_id: follower_id,
            allowed: false,
            mutual: false,
            comment: None,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "lists"]
#[belongs_to(User)]
pub struct List {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub comment: String,
    pub priority: i32,
    pub url: Option<String>,
    pub is_public: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl List {
    pub fn new(user_id: String, name: String, comment: String) -> List {
        List {
            id: Uuid::new_v4().to_string(),
            user_id: user_id,
            name: name,
            comment: comment,
            priority: 0,
            url: None,
            is_public: true,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
            updated_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "listmaps"]
#[belongs_to(List)]
#[belongs_to(Game)]
pub struct Listmap {
    pub id: String,
    pub list_id: String,
    pub game_id: i32,
}

impl Listmap {
    pub fn new(list_id: String, game_id: i32) -> Listmap {
        Listmap {
            id: Uuid::new_v4().to_string(),
            list_id: list_id,
            game_id: game_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "listlogs"]
#[belongs_to(List)]
#[belongs_to(Timeline)]
pub struct Listlog {
    pub id: String,
    pub timeline_id: String,
    pub list_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Listlog {
    pub fn new(timeline_id: String, list_id: String) -> Listlog {
        Listlog {
            id: Uuid::new_v4().to_string(),
            timeline_id: timeline_id,
            list_id: list_id,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "reviews"]
#[belongs_to(User)]
#[belongs_to(Game)]
pub struct Review {
    pub id: String,
    pub game_id: i32,
    pub user_id: String,
    pub es_user_id: String,
    pub tokuten: Option<i32>,
    pub tourokubi: Option<chrono::NaiveDateTime>,
    pub hitokoto: Option<String>,
    pub memo: Option<String>,
    pub netabare: Option<bool>,
    pub giveup: Option<bool>,
    pub possession: Option<bool>,
    pub play: Option<bool>,
    pub before_hitokoto: Option<String>,
    pub before_tokuten: Option<i32>,
    pub before_tourokubi: Option<chrono::NaiveDateTime>,
    pub display: Option<bool>,
    pub play_tourokubi: Option<chrono::NaiveDateTime>,
    pub display_unique_count: Option<i32>,
    pub sage: Option<bool>,
    pub before_purchase_will: Option<String>,
    pub before_sage: Option<bool>,
    pub total_play_time: Option<i32>,
    pub time_before_understanding_fun: Option<i32>,
    pub okazu_tokuten: Option<i32>,
    pub trial_version_hitokoto: Option<String>,
    pub trial_version_hitokoto_sage: Option<bool>,
    pub trial_version_hitokoto_tourokubi: Option<chrono::NaiveDateTime>,
    pub es_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Review {
    pub fn initialize() -> Review {
        Review {
            id: Uuid::new_v4().to_string(),
            game_id: 0,
            user_id: String::from(""),
            es_user_id: String::from(""),
            tokuten: None,
            tourokubi: None,
            hitokoto: None,
            memo: None,
            netabare: None,
            giveup: None,
            possession: None,
            play: None,
            before_hitokoto: None,
            before_tokuten: None,
            before_tourokubi: None,
            display: None,
            play_tourokubi: None,
            display_unique_count: None,
            sage: None,
            before_purchase_will: None,
            before_sage: None,
            total_play_time: None,
            time_before_understanding_fun: None,
            okazu_tokuten: None,
            trial_version_hitokoto: None,
            trial_version_hitokoto_sage: None,
            trial_version_hitokoto_tourokubi: None,
            es_id: None,
            created_at: Some(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)),
            updated_at: Some(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)),
        }
    }
    pub fn new(game_id: i32, user_id: String, es_user_id: String) -> Review {
        Review {
            id: Uuid::new_v4().to_string(),
            game_id: game_id,
            user_id: user_id,
            es_user_id: es_user_id,
            tokuten: None,
            tourokubi: None,
            hitokoto: None,
            memo: None,
            netabare: None,
            giveup: None,
            possession: None,
            play: None,
            before_hitokoto: None,
            before_tokuten: None,
            before_tourokubi: None,
            display: None,
            play_tourokubi: None,
            display_unique_count: None,
            sage: None,
            before_purchase_will: None,
            before_sage: None,
            total_play_time: None,
            time_before_understanding_fun: None,
            okazu_tokuten: None,
            trial_version_hitokoto: None,
            trial_version_hitokoto_sage: None,
            trial_version_hitokoto_tourokubi: None,
            es_id: None,
            created_at: Some(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)),
            updated_at: Some(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName, PartialEq)]
#[table_name = "reviewlogs"]
#[belongs_to(Review)]
#[belongs_to(Timeline)]
pub struct Reviewlog {
    pub id: String,
    pub timeline_id: String,
    pub review_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Reviewlog {
    pub fn new(timeline_id: String, review_id: String) -> Reviewlog {
        Reviewlog {
            id: Uuid::new_v4().to_string(),
            timeline_id: timeline_id,
            review_id: review_id,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[belongs_to(parent = User, foreign_key = "from_user_id")]
#[belongs_to(parent = User, foreign_key = "to_user_id")]
#[table_name = "messages"]
pub struct Message {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl Message {
    pub fn new(from_user_id: String, to_user_id: String, message: String) -> Message {
        Message {
            id: Uuid::new_v4().to_string(),
            from_user_id: from_user_id,
            to_user_id: to_user_id,
            message: message,
            created_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
            updated_at: chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable, Associations, Insertable, QueryableByName)]
#[table_name = "randomids"]
#[belongs_to(User)]
pub struct Randomid {
    pub id: String,
    pub user_id: String,
    pub purpose: i32, // 0 => through, 1 => timeline, 2 => direct, 3 => name, 4 => play, 5 => review, 6 => list
}

impl Randomid {
    pub fn new(user_id: String, purpose: i32) -> Randomid {
        Randomid {
            id: Uuid::new_v4().to_string(),
            user_id: user_id,
            purpose: purpose,
        }
    }
}

pub enum RandomPurpose {
    Throufh,
    FTimeline,
    FDirect,
    FName,
    FPlay,
    FReview,
    FList,
    FFollow,
}
