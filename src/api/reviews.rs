use actix_web::{web, Error, HttpResponse};
use super::super::actions::reviews;

pub async fn get_review(
    pools: web::Data<super::super::Pools>,
    review_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;
    let review_id: uuid::Uuid = review_id.into_inner().parse().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let review = web::block(move || reviews::find_review_by_uid(review_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(review) = review {
        Ok(HttpResponse::Ok().json(review))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No review found with uid: {}", review_id));
        Ok(res)
    }
}

pub async fn get_reviews(
    pools: web::Data<super::super::Pools>
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    // use web::block to offload blocking Diesel code without blocking server thread
    let review = web::block(move || reviews::find_reviews(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(review))
}

pub async fn add_review(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let new_reviews = super::super::actions::logics::scraping::reviews::get_all_reviews()
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
    let reviews = web::block(move || reviews::insert_new_reviews(new_reviews, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(reviews))
}

pub async fn add_all_review(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let new_reviews = super::super::actions::logics::scraping::reviews::get_all_reviews()
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
    let review = web::block(move || reviews::insert_new_reviews(new_reviews, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(review))
}
