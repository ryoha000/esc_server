use actix_web::{web, App, HttpServer};
use esc_server::api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::hello_world::hello_world))
            // .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
