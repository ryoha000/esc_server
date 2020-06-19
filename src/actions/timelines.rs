use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_timeline_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Timeline>, diesel::result::Error> {
    use crate::schema::timelines::dsl::*;

    let timeline = timelines
        .filter(id.eq(uid.to_string()))
        .first::<models::Timeline>(conn)
        .optional()?;

    Ok(timeline)
}

pub fn find_timelines(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Timeline>>, diesel::result::Error> {
    use crate::schema::timelines::dsl::*;

    let timeline = timelines
        .load::<models::Timeline>(conn)
        .optional()?;

    Ok(timeline)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_timeline(
    // prevent collision with `name` column imported inside the function
    new_timeline: models::Timeline,
    conn: &PgConnection,
) -> Result<models::Timeline, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::timelines::dsl::*;

    diesel::insert_into(timelines).values(&new_timeline).execute(conn)?;

    Ok(new_timeline)
}
