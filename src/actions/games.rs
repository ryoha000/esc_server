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

    // const BATCH_SIZE: i32 = 2000;
    // let len = new_games.len();
    // println!("{}", len);
    // let number_of_batch = len as i32 / BATCH_SIZE;
    // println!("{}", number_of_batch);
    // let mut split_games_vec: Vec<Vec<models::Game>> = Vec::new();
    // for i in 0..(number_of_batch + 1) {
    //     let mut split_games: Vec<models::Game> = Vec::new();
    //     for j in 0..BATCH_SIZE {
    //         match new_games.get((i * BATCH_SIZE + j) as usize) {
    //             Some(g) => split_games.push(g.clone()),
    //             _ => {}
    //         }
    //     }
    //     split_games_vec.push(split_games);
    // }

    let a = new_games.clone();
    for new_game in new_games {
        match diesel::insert_into(games).values(&new_game).execute(conn) {
            Ok(_) => {},
            e => {
                eprintln!("{:?}", e);
                eprintln!("{:?}", new_game);
            }
        };
    }

    Ok(a)
}
