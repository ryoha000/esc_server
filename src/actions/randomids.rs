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

pub fn get_randomids_by_user_ids (
    search_user_ids: Vec<String>,
    search_purpose: i32,
    conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
    let mut where_query = String::new();
    let _len = search_user_ids.len();
    for (i, id) in search_user_ids.iter().enumerate() {
        match i {
            _len => where_query.push_str(&(format!("user_id = \'{}\'", id))),
            _ => where_query.push_str(&(format!("user_id = \'{}\' OR ", id)))
        }
    }
    println!("{}", where_query);
    let query = format!("SELECT randomids.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join randomids on randomids.user_id = users.id WHERE purpose = {} AND ( {} );", search_purpose, where_query);
    println!("{}", query);
    let ids: Vec<models::User> = diesel::sql_query(query).load(conn)?;
    Ok(ids)
}
