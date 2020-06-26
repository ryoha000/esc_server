use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

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

pub fn insert_new_timeline(
    new_timeline: models::Timeline,
    conn: &PgConnection,
) -> Result<models::Timeline, diesel::result::Error> {
    use crate::schema::timelines::dsl::*;

    diesel::insert_into(timelines).values(&new_timeline).execute(conn)?;

    Ok(new_timeline)
}

pub fn insert_new_timelines(
    new_timelines: Vec<models::Timeline>,
    conn: &PgConnection,
) -> Result<Vec<models::Timeline>, diesel::result::Error> {
    use crate::schema::timelines::dsl::*;

    diesel::insert_into(timelines).values(&new_timelines).execute(conn)?;

    Ok(new_timelines)
}

pub fn find_timeline_with_game_by_timeline_id(
    search_timeline_id: String,
    conn: &PgConnection,
) -> Result<Option<(models::Timeline, models::Game)>, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    use crate::schema::timelines::dsl::*;

    let res = timelines
        .inner_join(games)
        .filter(crate::schema::timelines::id.eq(search_timeline_id))
        .first::<(models::Timeline, models::Game)>(conn)
        .optional()?;

    Ok(res)
}
