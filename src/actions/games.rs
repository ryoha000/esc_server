use diesel::prelude::*;

use super::super::models;
use chrono::Datelike;

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

pub fn find_games_limited(
    conn: &PgConnection,
) -> Result<Option<Vec<(i32, Option<String>)>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;

    let get_games = games
        .select((id, gamename))
        .load::<(i32, Option<String>)>(conn)
        .optional()?;

    Ok(get_games)
}

pub fn find_games_recent(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Game>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;

    let t = chrono::Local::today();
    let get_games = games
        .filter(sellday.gt(chrono::NaiveDate::from_ymd(t.year(), t.month(), t.day())))
        .filter(sellday.ne(chrono::NaiveDate::from_ymd(2030, 1, 1)))
        .load::<models::Game>(conn)
        .optional()?;

    Ok(get_games)
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

pub fn find_games_by_ids (
    search_ids: &Vec<i32>,
    conn: &PgConnection,
) -> Result<Vec<models::Game>, diesel::result::Error> {
    let mut where_query = String::new();
    let _len = search_ids.len();
    for (i, id) in search_ids.iter().enumerate() {
        if i == _len - 1 {
            where_query.push_str(&(format!("id = \'{}\'", id.to_string())))
        } else {
            where_query.push_str(&(format!("id = \'{}\' OR ", id.to_string())))
        }
    }
    let query = format!("SELECT * FROM games WHERE {}", where_query);
    let res_games: Vec<models::Game> = diesel::sql_query(query).load(conn)?;
    Ok(res_games)
}

pub fn insert_new_game(
    new_game: models::Game,
    conn: &PgConnection,
) -> Result<models::Game, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    diesel::insert_into(games).values(&new_game).execute(conn)?;

    Ok(new_game)
}

pub fn insert_new_games(
    new_games: Vec<models::Game>,
    conn: &PgConnection,
) -> Result<Vec<models::Game>, diesel::result::Error> {
    use crate::schema::games::dsl::*;

    const BATCH_SIZE: i32 = 2000;
    let len = new_games.len();
    println!("{}", len);
    let number_of_batch = len as i32 / BATCH_SIZE;
    println!("{}", number_of_batch);
    let mut split_games_vec: Vec<Vec<models::Game>> = Vec::new();
    for i in 0..(number_of_batch + 1) {
        let mut split_games: Vec<models::Game> = Vec::new();
        for j in 0..BATCH_SIZE {
            match new_games.get((i * BATCH_SIZE + j) as usize) {
                Some(g) => split_games.push(g.clone()),
                _ => {}
            }
        }
        split_games_vec.push(split_games);
    }

    for sg in split_games_vec {
        match diesel::insert_into(games).values(&sg).execute(conn) {
            Ok(_) => {},
            e => {
                eprintln!("{:?}", e);
                eprintln!("{:?}", sg);
            }
        };
    }

    // for new_game in &new_games {
    //     match diesel::insert_into(games).values(new_game).execute(conn) {
    //         Ok(_) => {},
    //         e => {
    //             eprintln!("{:?}", e);
    //             eprintln!("{:?}", new_game);
    //         }
    //     };
    // }

    Ok(new_games)
}
