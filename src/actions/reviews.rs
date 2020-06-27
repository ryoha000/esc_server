use diesel::prelude::*;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_review_by_id(
    review_id: String,
    conn: &PgConnection,
) -> Result<Option<models::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews
        .filter(id.eq(review_id))
        .first::<models::Review>(conn)
        .optional()?;

    Ok(review)
}

pub fn find_reviews(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Review>>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews
        .load::<models::Review>(conn)
        .optional()?;

    Ok(review)
}

pub fn insert_new_review(
    new_review: models::Review,
    conn: &PgConnection,
) -> Result<models::Review, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    diesel::insert_into(reviews).values(&new_review).execute(conn)?;

    Ok(new_review)
}

pub fn insert_new_reviews(
    new_reviews: Vec<models::Review>,
    conn: &PgConnection,
) -> Result<Vec<models::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    for r in &new_reviews {
        match diesel::insert_into(reviews).values(r).execute(conn) {
            Ok(_) => {},
            e => {
                eprintln!("{:?}", e);
                eprintln!("{:?}", r);
            }
        };
        
    }

    Ok(new_reviews)
}
