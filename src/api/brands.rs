use actix_web::{web, Error, HttpResponse};
use super::super::actions::brands;
use std::ops::DerefMut;
use serde::{Deserialize, Serialize};

pub async fn get_brand(
    pools: web::Data<super::super::Pools>,
    brand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    let brand_id = brand_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let brand = web::block(move || brands::find_brand_by_id(brand_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(brand) = brand {
        Ok(HttpResponse::Ok().json(brand))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No brand found with uid: {}", brand_id));
        Ok(res)
    }
}

pub async fn get_brands(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let brand = web::block(move || brands::find_brands(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(brand))
}

pub async fn add_brand(
    pools: web::Data<super::super::Pools>
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

    let new_brands = super::super::actions::logics::scraping::brands::get_latest_brands_by_id(max_id)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut new_max_id = 0;
    for r in &new_brands {
        if new_max_id < r.id {
            new_max_id = r.id
        }
    }
    r2d2_redis::redis::cmd("SET").arg("max_game_id").arg(format!("{:?}", new_max_id)).query(redis_conn.deref_mut()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if new_brands.len() == 0 { return Ok(HttpResponse::Ok().body("there is no new game")) }

    println!("{:?}", new_brands.len());
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    // println!("{}", &form.)

    // use web::block to offload blocking Diesel code without blocking server thread
    let brands = web::block(move || brands::insert_new_brands(new_brands, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(brands))
}

pub async fn add_id_brand(
    pools: web::Data<super::super::Pools>,
    brand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let new_brand = super::super::actions::logics::scraping::brands::get_latest_brand_by_id(brand_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from poolss");
        HttpResponse::InternalServerError().finish()
    })?;
    // println!("{}", &form.)

    // use web::block to offload blocking Diesel code without blocking server thread
    let brand = web::block(move || brands::insert_new_brand(new_brand, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(brand))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStruct {
    pub password: String,
    pub data: String,
}

pub async fn update_all_brands(
    pools: web::Data<super::super::Pools>,
    form: web::Json<UpdateStruct>,
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

    // 今ある全てのブランドを取得
    let new_brands = actions::logics::scraping::brands::get_all_brands(form.data.clone())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("finish get brands: {}", new_brands.len());

    web::block(move || actions::brands::delete_all_brands(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _ = web::block(move || actions::brands::insert_new_brands(new_brands, &conn))
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

    println!("finish setup brands");
    Ok(HttpResponse::Ok().body("brands updated"))
}
