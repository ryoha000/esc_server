use actix_web::{web, Error, HttpResponse};
use super::super::actions::games;

pub async fn get_game(
    pool: web::Data<super::super::DbPool>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let game_id = game_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let game = web::block(move || games::find_game_by_id(game_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(game) = game {
        Ok(HttpResponse::Ok().json(game))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No game found with uid: {}", game_id));
        Ok(res)
    }
}

pub async fn get_games(
    pool: web::Data<super::super::DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let game = web::block(move || games::find_games(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(game))
}

pub async fn add_game(
    pool: web::Data<super::super::DbPool>
) -> Result<HttpResponse, Error> {
    let id = super::super::login();
    let pass = super::super::login1();
    let admin_header = super::super::actions::logics::es_login::es_login(&id, &pass).await;
    println!("{:?}", admin_header);
    let new_game = super::super::actions::logics::scraping::get_test_game(admin_header).await;

    let conn = pool.get().expect("couldn't get db connection from pool");
    // println!("{}", &form.)

    // use web::block to offload blocking Diesel code without blocking server thread
    let game = web::block(move || games::insert_new_game(new_game, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(game))
}