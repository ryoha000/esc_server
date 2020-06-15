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

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
