#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod schema;
pub mod models;
pub mod actions;
pub mod middleware;
pub mod ws_actor;


use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::{web, HttpResponse};
use actix::prelude::*;
use std::ops::DerefMut;

#[cfg(test)]
pub mod tests;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type RedisPool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;

#[derive(Clone)]
pub struct Pools {
    pub db: DbPool,
    pub redis: RedisPool,
}

pub fn get_db_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_redis_url() -> String {
    dotenv().ok();

    env::var("REDIS_URL").expect("REDIS_URL must be set")
}

pub fn login() -> String {
    dotenv().ok();

    env::var("ADMIN_USER_NAME").expect("ADMIN_USER_NAME must be set")

}

pub fn login1() -> String {
    dotenv().ok();

    env::var("ADMIN_USER_PASS").expect("ADMIN_USER_PASS must be set")
}

pub async fn db_setup(pools: &Pools) {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    }).unwrap();

    let mut new_user = models::User::new();
    new_user.id = String::from("");
    new_user.es_user_id = String::from("批評空間のユーザー");
    new_user.display_name = String::from("批評空間のユーザー");
    let _ = actions::users::insert_new_user(new_user, &conn).unwrap();

    // 今ある全てのゲームを取得
    let new_games = actions::logics::scraping::games::get_all_games()
        .await
        .unwrap();

    let _ = actions::games::insert_new_games(new_games, &conn).unwrap();

    // 今ある全てのブランドを取得
    let new_brands = actions::logics::scraping::brands::get_all_brands()
        .await
        .unwrap();

    let _ = actions::brands::insert_new_brands(new_brands, &conn).unwrap();

    let new_datas = actions::logics::scraping::games::get_all_data()
        .await
        .unwrap();

    let mut redis_conn = pools.redis.get().unwrap();

    let mut new_max_id = 0;

    for r in new_datas {
        r2d2_redis::redis::cmd("SET").arg(format!("game_id:{:?}", r.id)).arg(format!("{:?} {:?} {:?}", r.median, r.stdev, r.count2)).query::<()>(redis_conn.deref_mut()).unwrap();
        if new_max_id < r.id {
            new_max_id = r.id
        }
    }
    r2d2_redis::redis::cmd("SET").arg("max_game_id").arg(format!("{:?}", new_max_id)).query::<()>(redis_conn.deref_mut()).unwrap();
}
