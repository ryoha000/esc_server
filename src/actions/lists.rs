use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

pub fn find_list_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::List>, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    let list = lists
        .filter(id.eq(uid.to_string()))
        .first::<models::List>(conn)
        .optional()?;

    Ok(list)
}

pub fn find_lists(
    conn: &PgConnection,
) -> Result<Option<Vec<models::List>>, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    let list = lists
        .load::<models::List>(conn)
        .optional()?;

    Ok(list)
}

pub fn insert_new_list(
    new_list: models::List,
    conn: &PgConnection,
) -> Result<models::List, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    diesel::insert_into(lists).values(&new_list).execute(conn)?;

    Ok(new_list)
}
