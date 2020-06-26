use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_reviewlog_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Reviewlog>, diesel::result::Error> {
    use crate::schema::reviewlogs::dsl::*;

    let reviewlog = reviewlogs
        .filter(id.eq(uid.to_string()))
        .first::<models::Reviewlog>(conn)
        .optional()?;

    Ok(reviewlog)
}

pub fn find_reviewlogs(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Reviewlog>>, diesel::result::Error> {
    use crate::schema::reviewlogs::dsl::*;

    let reviewlog = reviewlogs
        .load::<models::Reviewlog>(conn)
        .optional()?;

    Ok(reviewlog)
}

pub fn insert_new_reviewlog(
    new_reviewlog: models::Reviewlog,
    conn: &PgConnection,
) -> Result<models::Reviewlog, diesel::result::Error> {
    use crate::schema::reviewlogs::dsl::*;

    diesel::insert_into(reviewlogs).values(&new_reviewlog).execute(conn)?;

    Ok(new_reviewlog)
}

pub fn insert_new_reviewlogs(
    new_reviewlogs: Vec<models::Reviewlog>,
    conn: &PgConnection,
) -> Result<Vec<models::Reviewlog>, diesel::result::Error> {
    use crate::schema::reviewlogs::dsl::*;

    diesel::insert_into(reviewlogs).values(&new_reviewlogs).execute(conn)?;

    Ok(new_reviewlogs)
}

pub fn find_review_by_timeline_id(
    search_timeline_id: String,
    conn: &PgConnection,
) -> Result<Option<(models::Reviewlog, models::Review)>, diesel::result::Error> {
    use crate::schema::reviewlogs::dsl::*;
    use crate::schema::reviews::dsl::*;

    let reviewlog = reviewlogs
        .inner_join(reviews)
        .filter(timeline_id.eq(search_timeline_id))
        .first::<(models::Reviewlog, models::Review)>(conn)
        .optional()?;

    Ok(reviewlog)
}
