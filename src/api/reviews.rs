use actix_web::{web, Error, HttpResponse};
use super::super::actions::reviews;
use super::super::actions::users;
use super::super::actions::timelines;
use super::super::actions::reviewlogs;
use super::super::models;
use std::ops::DerefMut;

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

pub async fn add_recent_reviews(
    pools: web::Data<super::super::Pools>,
) -> Result<HttpResponse, Error> {
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut redis_conn = pools.redis.get().map_err(|_| {
        eprintln!("couldn't get redis connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let mut max_id: i32 = 0;
    match r2d2_redis::redis::cmd("GET").arg("max_review_id").query(redis_conn.deref_mut()) {
        Ok(res) => {
            let max_id_string:String = res;
            max_id = max_id_string.parse().map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?
        },
        _ => max_id = 2013000
    }


    let new_reviews = super::super::actions::logics::scraping::reviews::get_recent_reviews(max_id)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut new_max_id = 0;

    for r in &new_reviews {
        if let Some(id) = r.es_id {
            if new_max_id < id {
                new_max_id = id
            }
        }
    }

    let mut user_ids: Vec<(String, String)> = Vec::new();

    match web::block(move || users::get_all_user_id(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })? {
            Some(res) => user_ids = res,
            _ => {}
        }

    let mut insert_reviews: Vec<models::Review> = Vec::new();

    for r in new_reviews {
        let mut _review = r;
        for ids in &user_ids {
            let (id, es_user_id) = ids;
            let id = id.clone();
            if es_user_id == &_review.es_user_id {
                _review.user_id = id;
                break;
            }
        }
        insert_reviews.push(_review);
    }

    r2d2_redis::redis::cmd("SET").arg("max_review_id").arg(new_max_id).query(redis_conn.deref_mut()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let reviews = web::block(move || reviews::insert_new_reviews(insert_reviews, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut new_timelines: Vec<models::Timeline> = Vec::new();
    let mut new_reviewlogs: Vec<models::Reviewlog> = Vec::new();
    for cr in &reviews {
        let _timeline = models::Timeline::new(cr.user_id.clone(), cr.game_id, 2);
        new_reviewlogs.push(models::Reviewlog::new(_timeline.id.clone(), cr.id.clone()));
        new_timelines.push(_timeline);
    }
    
    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _timelines = web::block(move || timelines::insert_new_timelines(new_timelines, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pools.db.get().map_err(|_| {
        eprintln!("couldn't get db connection from pools");
        HttpResponse::InternalServerError().finish()
    })?;

    let _ = web::block(move || reviewlogs::insert_new_reviewlogs(new_reviewlogs, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(reviews))
}
