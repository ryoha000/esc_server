use diesel::prelude::*;

use super::super::models;
use chrono::Datelike;

pub fn find_game_by_id(
    _id: i32,
    conn: &PgConnection,
) -> Result<Option<(models::Game, models::Brand)>, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    use crate::schema::brands::dsl::*;

    let game = games
        .inner_join(brands)
        .filter(crate::schema::games::id.eq(_id))
        .first::<(models::Game, models::Brand)>(conn)
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
) -> Result<Option<Vec<(models::Game, models::Brand)>>, diesel::result::Error> {
    use crate::schema::games::dsl::*;
    use crate::schema::brands::dsl::*;

    let t = chrono::Local::today();
    let get_games = games
        .inner_join(brands)
        .filter(sellday.gt(chrono::NaiveDate::from_ymd(t.year(), t.month(), t.day())))
        .filter(sellday.ne(chrono::NaiveDate::from_ymd(2030, 1, 1)))
        .load::<(models::Game, models::Brand)>(conn)
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
    use crate::schema::games::dsl::*;

    let res_games = games
        .filter(id.eq_any(search_ids))
        .load(conn)?;

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

pub fn delete_all_games(
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::games::dsl::*;

    diesel::delete(games).execute(conn)?;
    Ok(())
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
