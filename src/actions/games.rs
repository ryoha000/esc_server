use diesel::prelude::*;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_game_by_id(
    _id: i32,
    conn: &PgConnection,
) -> Result<Option<models::Game>, diesel::result::Error> {
    use crate::schema::games::dsl::*;

    let game = games
        .filter(id.eq(_id))
        .first::<models::Game>(conn)
        .optional()?;

    Ok(game)
}

pub fn find_games(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Game>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;

    let game = games
        .load::<models::Game>(conn)
        .optional()?;

    Ok(game)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_game(
    new_game: models::Game,
    conn: &PgConnection,
) -> Result<models::Game, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::games::dsl::*;
    diesel::insert_into(games).values(&new_game).execute(conn)?;

    Ok(new_game)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_games(
    new_games: Vec<models::Game>,
    conn: &PgConnection,
) -> Result<Vec<models::Game>, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::games::dsl::*;
    diesel::insert_into(games).values(&new_games).execute(conn)?;

    Ok(new_games)
}
