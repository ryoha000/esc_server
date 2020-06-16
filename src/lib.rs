pub mod api;
pub mod schema;
pub mod models;
pub mod actions;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[cfg(test)]
pub mod tests;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn login() -> String {
    dotenv().ok();

    env::var("ADMIN_USER_NAME").expect("ADMIN_USER_NAME must be set")

}

pub fn login1() -> String {
    dotenv().ok();

    env::var("ADMIN_USER_PASS").expect("ADMIN_USER_PASS must be set")
}
