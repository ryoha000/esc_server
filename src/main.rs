use actix_web::{web, App, HttpServer};
use esc_server::api;
use esc_server;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use actix_web::middleware::Logger;
use env_logger;
use env_logger::Env;

extern crate redis;
extern crate r2d2_redis;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url: String = esc_server::get_db_url();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create postgres pool.");

    let redis_url = esc_server::get_redis_url();
    let redis_manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    let redis_pool = r2d2_redis::r2d2::Pool::builder()
        .build(redis_manager)
        .expect("Failed to create redis pool.");

    let pools = esc_server::Pools {
        db: pool,
        redis: redis_pool,
    };

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .data(pools.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(api::hello_world::hello_world))
            .route("/users/{user_id}", web::get().to(api::users::get_user))
            .route("/users", web::get().to(api::users::get_users))
            .route("/users", web::post().to(api::users::signup))
            .route("/brands", web::get().to(api::brands::get_brands))
            .route("/brands/{brand_id}", web::get().to(api::brands::get_brand))
            // for test
            .route("/brands", web::post().to(api::brands::add_brand))
            // for test
            .route("brands/{brand_id}", web::post().to(api::brands::add_id_brand))
            .route("/games", web::get().to(api::games::get_games))
            .route("/games/{game_id}", web::get().to(api::games::get_game))
            // for test
            .route("/games", web::post().to(api::games::add_game))
            // for test
            .route("games/{game_id}", web::post().to(api::games::add_id_game))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
