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
    pool: web::Data<super::super::DbPool>,
) -> Result<HttpResponse, Error> {
    let new_games = super::super::actions::logics::scraping::games::get_all_games()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pool.get().expect("couldn't get db connection from pool");
    // println!("{}", &form.)

    // use web::block to offload blocking Diesel code without blocking server thread
    let games = web::block(move || games::insert_new_games(new_games, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(games))
}

pub async fn add_id_game(
    pool: web::Data<super::super::DbPool>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let new_game = super::super::actions::logics::scraping::games::get_latest_game_by_id(game_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

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