use diesel::prelude::*;
use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    id: String,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(id))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_name(
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

pub fn find_users(
    conn: &PgConnection,
) -> Result<Option<Vec<models::User>>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .load::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    new_user: models::User,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;
    use crate::schema::randomids::dsl::*;

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut new_randomids: Vec<models::Randomid> = Vec::new();
    for i in 0..7 {
        new_randomids.push(models::Randomid::new(new_user.id.clone(), i));
    }
    diesel::insert_into(randomids).values(&new_randomids).execute(conn)?;
    Ok(new_user)
}

pub fn get_all_user_id(
    conn: &PgConnection,
) -> Result<Option<Vec<(String, String)>>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .select((id, es_user_id))
        .load::<(String, String)>(conn)
        .optional()?;

    Ok(user)
}
