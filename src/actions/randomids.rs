use diesel::prelude::*;
use uuid::Uuid;


use super::super::models;

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
    search_user_id: String,
    search_purpose: i32,
    conn: &PgConnection,
) -> Result<models::Randomid, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;

    let randomid = randomids
        .filter(user_id.eq(search_user_id))
        .filter(purpose.eq(search_purpose))
        .first::<models::Randomid>(conn)?;

    Ok(randomid)
}

pub fn get_user_by_id (
    r_uuid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;
    use crate::schema::users::dsl::*;
    let _user = randomids
        .inner_join(users)
        .filter(crate::schema::randomids::id.eq(r_uuid.to_string()))
        .first::<(models::Randomid, models::User)>(conn)
        .optional()?;

    match _user {
        Some((_, user)) => Ok(user),
        None => Err(diesel::result::Error::NotFound)
    }
}

pub fn get_randomids_by_user_ids (
    search_user_ids: Vec<String>,
    search_purpose: i32,
    conn: &PgConnection,
) -> Result<Vec<(models::Randomid, models::User)>, diesel::result::Error> {
    use crate::schema::randomids::dsl::*;
    use crate::schema::users::dsl::*;

    let rid_users = randomids
        .inner_join(users)
        .filter(user_id.eq_any(search_user_ids))
        .load::<(models::Randomid, models::User)>(conn)?;

    Ok(rid_users)
}
