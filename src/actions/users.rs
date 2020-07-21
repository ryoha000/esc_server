use diesel::prelude::*;
use super::super::models;

pub fn find_user_by_uid(
    user_id: String,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_user_by_es_name(
    search_name: String,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(es_user_id.eq(search_name))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_user_by_name(
    search_name: String,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(name.eq(search_name))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_users(
    conn: &PgConnection,
) -> Result<Option<Vec<models::User>>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .load::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    new_user: models::User,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use crate::schema::randomids::dsl::*;

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut new_randomids: Vec<models::Randomid> = Vec::new();
    // throughはそのままだから
    new_randomids.push(models::Randomid {
        id: new_user.id.clone(),
        user_id: new_user.id.clone(),
        purpose: models::RandomPurpose::Throufh as i32,
    });
    for i in 1..8 {
        new_randomids.push(models::Randomid::new(new_user.id.clone(), i));
    }
    diesel::insert_into(randomids).values(&new_randomids).execute(conn)?;
    Ok(new_user)
}

pub fn update_user(
    user_id: String,
    update_user: &models::User,
    conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let update_row = diesel::update(users.filter(id.eq(user_id)))
        .set(update_user)
        .load::<models::User>(conn);

    update_row
}

pub fn get_all_user_id(
    conn: &PgConnection,
) -> Result<Option<Vec<(String, Option<String>)>>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .select((id, es_user_id))
        .load::<(String, Option<String>)>(conn)
        .optional()?;

    Ok(user)
}
