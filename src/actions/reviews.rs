use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_review_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews
        .filter(id.eq(uid.to_string()))
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

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_review(
    // prevent collision with `name` column imported inside the function
    new_review: models::Review,
    conn: &PgConnection,
) -> Result<models::Review, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::reviews::dsl::*;

    diesel::insert_into(reviews).values(&new_review).execute(conn)?;

    Ok(new_review)
}

pub fn insert_new_reviews(
    new_reviews: Vec<models::Review>,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    for r in new_reviews {
        match diesel::insert_into(reviews).values(&r).execute(conn) {
            Ok(_) => {},
            e => {
                eprintln!("{:?}", e);
                eprintln!("{:?}", r);
            }
        };
        
    }

    Ok(())
}
