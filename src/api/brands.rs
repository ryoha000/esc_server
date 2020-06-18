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
    let new_brands = super::super::actions::logics::scraping::brands::get_all_brands()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("{:?}", new_brands.len());
    let conn = pool.get().expect("couldn't get db connection from pool");
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
    pool: web::Data<super::super::DbPool>,
    brand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let new_brand = super::super::actions::logics::scraping::brands::get_latest_brand_by_id(brand_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

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
