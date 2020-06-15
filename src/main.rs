use actix_web::{web, App, HttpServer};
use esc_server::api;
use esc_server;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url: String = esc_server::get_db_url();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(api::hello_world::hello_world))
            .route("/users/{user_id}", web::get().to(api::users::get_user))
            .route("/users", web::get().to(api::users::get_users))
            .route("/users", web::post().to(api::users::signup))
            // .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
