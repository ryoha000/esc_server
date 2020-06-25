use diesel::prelude::*;
use uuid::Uuid;


use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_randomid_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Randomid>, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    let randomid = randomids
        .filter(id.eq(uid.to_string()))
        .first::<models::Randomid>(conn)
        .optional()?;

    Ok(randomid)
}

pub fn find_randomids(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Randomid>>, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    let randomid = randomids
        .load::<models::Randomid>(conn)
        .optional()?;

    Ok(randomid)
}

pub fn insert_new_randomid(
    new_randomid: models::Randomid,
    conn: &PgConnection,
) -> Result<models::Randomid, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    diesel::insert_into(randomids).values(&new_randomid).execute(conn)?;

    Ok(new_randomid)
}

pub fn insert_new_randomids(
    new_randomids: Vec<models::Randomid>,
    conn: &PgConnection,
) -> Result<Vec<models::Randomid>, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    diesel::insert_into(randomids).values(&new_randomids).execute(conn)?;

    Ok(new_randomids)
}

pub fn get_randomid_by_user_id (
    user_id: String,
    purpose: i32,
    conn: &PgConnection,
) -> Result<Option<models::Randomid>, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    let randomid = randomids
        .filter(user_id.eq(user_id))
        .filter(purpose.eq(purpose))
        .first::<models::Randomid>(conn)
        .optional()?;

    Ok(randomid)
}

pub fn get_user_by_id (
    id: String,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    let query = format!("SELECT users.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join randomids on randomids.user_id = users.id WHERE randomids.id = \'{}\';", id);
    let user: Vec<models::User> = diesel::sql_query(query).load(conn)?;

    match user.get(0) {
        Some(user) => Ok(user.clone()),
        None => Err(diesel::result::Error::NotFound)
    }
}