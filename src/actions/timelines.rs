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

pub fn insert_new_timelines_each(
    new_timelines: Vec<models::Timeline>,
    conn: &PgConnection,
) -> Result<Vec<models::Timeline>, diesel::result::Error> {
    use crate::schema::timelines::dsl::*;

    for nt in &new_timelines {
        match diesel::insert_into(timelines).values(nt).execute(conn) {
            Ok(_) => {},
            e => {
                eprintln!("{:?}", e);
                eprintln!("{:?}", nt);
            }
        }
    }

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

pub fn find_timelines_with_game_of_limit20(
    offset_num: i64,
    conn: &PgConnection,
) -> Result<Option<Vec<(models::Timeline, models::Game)>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    use crate::schema::timelines::dsl::*;

    let res = timelines
        .inner_join(games)
        .order(crate::schema::timelines::created_at.desc())
        .offset(offset_num * 20)
        .limit(20)
        .load::<(models::Timeline, models::Game)>(conn)
        .optional()?;

    Ok(res)
}

pub fn find_timelines_with_game_by_user_id_and_type_with_limit(
    _user_id: String,
    _log_type: i32,
    _limit: i64,
    conn: &PgConnection,
) -> Result<Option<Vec<(models::Timeline, models::Game)>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    use crate::schema::timelines::dsl::*;

    let res = timelines
        .inner_join(games)
        .order(crate::schema::timelines::created_at.desc())
        .filter(user_id.eq(_user_id)
            .and(log_type.eq(_log_type)))
        .limit(_limit)
        .load::<(models::Timeline, models::Game)>(conn)
        .optional()?;

    Ok(res)
}