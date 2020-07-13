use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

pub fn find_listmap_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Listmap>, diesel::result::Error> {
    use crate::schema::listmaps::dsl::*;

    let listmap = listmaps
        .filter(id.eq(uid.to_string()))
        .first::<models::Listmap>(conn)
        .optional()?;

    Ok(listmap)
}

pub fn find_listmaps(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Listmap>>, diesel::result::Error> {
    use crate::schema::listmaps::dsl::*;

    let listmap = listmaps
        .load::<models::Listmap>(conn)
        .optional()?;

    Ok(listmap)
}

pub fn insert_new_listmap(
    new_listmap: models::Listmap,
    conn: &PgConnection,
) -> Result<models::Listmap, diesel::result::Error> {
    use crate::schema::listmaps::dsl::*;

    diesel::insert_into(listmaps).values(&new_listmap).execute(conn)?;

    Ok(new_listmap)
}

pub fn insert_new_listmaps(
    new_listmaps: Vec<models::Listmap>,
    conn: &PgConnection,
) -> Result<Vec<models::Listmap>, diesel::result::Error> {
    use crate::schema::listmaps::dsl::*;

    diesel::insert_into(listmaps).values(&new_listmaps).execute(conn)?;

    Ok(new_listmaps)
}

pub fn delete_listmaps_by_list_id_and_list_map_ids(
    delete_list_id: String,
    game_ids: Vec<i32>,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::listmaps::dsl::*;

    diesel::delete(listmaps.filter(game_id.eq_any(game_ids).and(list_id.eq(delete_list_id)))).execute(conn)?;

    Ok(())
}
