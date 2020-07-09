use diesel::prelude::*;
use uuid::Uuid;
use super::super::models;
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ListWithGames {
    list: Option<models::List>,
    games: Option<Vec<models::Game>>,
}

pub fn find_list_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<ListWithGames, diesel::result::Error> {
    let get_game_query = format!("SELECT games.id, games.gamename, games.furigana, games.sellday, games.brand_id, games.comike, games.shoukai, games.model, games.erogame, games.banner_url, games.gyutto_id, games.dmm, games.dmm_genre, games.dmm_genre_2, games.erogametokuten, games.total_play_time_median, games.time_before_understanding_fun_median, games.dlsite_id, games.dlsite_domain, games.trial_url, games.okazu, games.axis_of_soft_or_hard, games.genre, games.twitter, games.digiket, games.twitter_data_widget_id, games.masterup, games.steam, games.dlsite_rental, games.dmm_subsc, games.surugaya_1, games.scheduled_date FROM (listmaps INNER JOIN lists ON lists.id = listmaps.list_id) INNER JOIN games ON games.id = listmaps.game_id WHERE lists.id = \'{}\'", uid.to_string());
    let games: Option<Vec<models::Game>> = diesel::sql_query(get_game_query).load(conn).optional()?;

    use crate::schema::lists::dsl::*;

    let list = lists
        .filter(id.eq(uid.to_string()))
        .first::<models::List>(conn)
        .optional()?;

    let res = ListWithGames {
        list: list,
        games: games
    };

    Ok(res)
}

pub fn update_list_by_id(
    new_list: &models::List,
    conn: &PgConnection,
) -> Result<Vec<models::List>, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    let update_row = diesel::update(lists.filter(id.eq(new_list.id.clone())))
        .set(new_list)
        .load::<models::List>(conn)?;

    Ok(update_row)
}

pub fn find_simple_list_by_uid(
    uid: String,
    conn: &PgConnection,
) -> Result<Option<models::List>, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    let list = lists
        .filter(id.eq(uid))
        .first::<models::List>(conn)
        .optional()?;

    Ok(list)
}

pub fn find_simple_lists_by_user_id(
    search_user_id: String,
    conn: &PgConnection,
) -> Result<Option<Vec<models::List>>, diesel::result::Error> {
    use crate::schema::lists::dsl::*;

    let list = lists
        .filter(user_id.eq(search_user_id))
        .load::<models::List>(conn)
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
