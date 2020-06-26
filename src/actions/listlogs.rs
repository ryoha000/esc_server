use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_listlog_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Listlog>, diesel::result::Error> {
    use crate::schema::listlogs::dsl::*;

    let listlog = listlogs
        .filter(id.eq(uid.to_string()))
        .first::<models::Listlog>(conn)
        .optional()?;

    Ok(listlog)
}

pub fn find_listlogs(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Listlog>>, diesel::result::Error> {
    use crate::schema::listlogs::dsl::*;

    let listlog = listlogs
        .load::<models::Listlog>(conn)
        .optional()?;

    Ok(listlog)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_listlog(
    // prevent collision with `name` column imported inside the function
    new_listlog: models::Listlog,
    conn: &PgConnection,
) -> Result<models::Listlog, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::listlogs::dsl::*;

    diesel::insert_into(listlogs).values(&new_listlog).execute(conn)?;

    Ok(new_listlog)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_listlogs(
    new_listlogs: Vec<models::Listlog>,
    conn: &PgConnection,
) -> Result<Vec<models::Listlog>, diesel::result::Error> {
    use crate::schema::listlogs::dsl::*;

    diesel::insert_into(listlogs).values(&new_listlogs).execute(conn)?;

    Ok(new_listlogs)
}

pub fn find_list_by_timeline_id(
    search_timeline_id: String,
    conn: &PgConnection,
) -> Result<Option<(models::Listlog, models::List)>, diesel::result::Error> {
    use crate::schema::listlogs::dsl::*;
    use crate::schema::lists::dsl::*;

    let res = listlogs
        .inner_join(lists)
        .filter(timeline_id.eq(search_timeline_id))
        .first::<(models::Listlog, models::List)>(conn)
        .optional()?;

    Ok(res)
}
