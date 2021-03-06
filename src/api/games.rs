use actix_web::{web, Error, HttpResponse};
use super::super::actions::games;
use super::super::models;
use std::ops::DerefMut;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimalGame {
    pub id: i32,
    pub gamename: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameAndBrand {
    pub game: models::Game,
    pub brand: models::Brand,
}

pub async fn get_game(
    pools: web::Data<super::super::Pools>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    let game_id = game_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    if let Some((game, brand)) = web::block(move || games::find_game_by_id(game_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })? {
            return Ok(HttpResponse::Ok().json(GameAndBrand { game: game, brand: brand }))
        }

    let res = HttpResponse::NotFound()
        .body(format!("No game found with uid: {}", game_id));
    Ok(res)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameIDs {
    pub ids: Vec<i32>,
}

pub async fn get_games(
    pools: web::Data<super::super::Pools>,
    form: web::Json<GameIDs>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let ids = form.ids.clone();
    // use web::block to offload blocking Diesel code without blocking server thread
    let games = web::block(move || games::find_games_by_ids(&ids, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(games))
}

pub async fn get_minimal_games(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let games = web::block(move || games::find_games_limited(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut minimal_games: Vec<MinimalGame> = Vec::new();
    if let Some(_games) = games {
        for game in _games {
            let mg = MinimalGame {
                id: game.0,
                gamename: game.1,
            };
            minimal_games.push(mg);
        }
    }
    Ok(HttpResponse::Ok().json(minimal_games))
}

pub async fn add_game(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut max_id: i32 = 0;
    match r2d2_redis::redis::cmd("GET").arg("max_game_id").query(redis_conn.deref_mut()) {
        Ok(res) => {
            let max_id_string:String = res;
            max_id = max_id_string.parse().map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?
        },
        _ => {}
    }

    let new_games = super::super::actions::logics::scraping::games::get_latest_games_by_id(max_id)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut new_max_id = 0;
    for r in &new_games {
        if new_max_id < r.id {
            new_max_id = r.id
        }
    }
    r2d2_redis::redis::cmd("SET").arg("max_game_id").arg(format!("{:?}", new_max_id)).query(redis_conn.deref_mut()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if new_games.len() == 0 { return Ok(HttpResponse::Ok().body("there is no new game")) }

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    
    let games = web::block(move || games::insert_new_games(new_games, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(games))
}

pub async fn add_id_game(
    pools: web::Data<super::super::Pools>,
    game_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let new_game = super::super::actions::logics::scraping::games::get_latest_game_by_id(game_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
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

pub async fn get_recent_games(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let games = web::block(move || games::find_games_recent(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(games))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub password: String,
}

pub async fn update_all_games(
    pools: web::Data<super::super::Pools>,
    form: web::Json<Password>,
) -> Result<HttpResponse, Error> {
    let pass = super::super::root_pass();
    if pass != form.password {
        return Ok(HttpResponse::Forbidden().body("this method allowed only admin"))
    }
    use super::super::actions;
    use std::process::Command;

    Command::new("diesel")
        .args(&["migration", "run"])
        .spawn()
        .map_err(|_| {
            eprintln!("failed to start `diesel migration run`");
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    
    // 今ある全てのゲームを取得
    let new_games = actions::logics::scraping::games::get_all_games()
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    println!("finish get games: {}", new_games.len());

    web::block(move || actions::games::delete_all_games(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _ = web::block(move || actions::games::insert_new_games(new_games, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Command::new("diesel")
        .args(&["migration", "revert"])
        .spawn()
        .map_err(|_| {
            eprintln!("failed to start `diesel migration revert`");
            HttpResponse::InternalServerError().finish()
        })?;

    println!("finish setup games");
    Ok(HttpResponse::Ok().body("games updated"))
}
