use actix_web::{web, Error, HttpResponse};
use super::super::actions::brands;

pub async fn get_brand(
    pool: web::Data<super::super::DbPool>,
    brand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
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
    pool: web::Data<super::super::DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

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
    pool: web::Data<super::super::DbPool>
) -> Result<HttpResponse, Error> {
    let id = super::super::login();
    let pass = super::super::login1();
    let admin_header = super::super::actions::logics::es_login::es_login(&id, &pass).await;
    println!("{:?}", admin_header);
    let new_brand = super::super::actions::logics::scraping::get_all_games(admin_header).await;

    let conn = pool.get().expect("couldn't get db connection from pool");
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