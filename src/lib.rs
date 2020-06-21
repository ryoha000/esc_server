#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod schema;
pub mod models;
pub mod actions;
pub mod middleware;


use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

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
