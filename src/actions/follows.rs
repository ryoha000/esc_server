use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

pub fn find_follow_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Follow>, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    let follow = follows
        .filter(id.eq(uid.to_string()))
        .first::<models::Follow>(conn)
        .optional()?;

    Ok(follow)
}

pub fn find_follows(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Follow>>, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    let follow = follows
        .load::<models::Follow>(conn)
        .optional()?;

    Ok(follow)
}

pub fn insert_new_follow(
    new_follow: models::Follow,
    conn: &PgConnection,
) -> Result<models::Follow, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    diesel::insert_into(follows).values(&new_follow).execute(conn)?;

    Ok(new_follow)
}

// そのUserがフォローされてる相手を取得
pub fn find_followers_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<Vec<models::User>>, diesel::result::Error> {
    // TODO: ちゃんとdieselでかく
    let query = format!("SELECT users.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join follows on follows.follower_id = users.id WHERE follows.allowed = true AND follows.followee_id = \'{}\';", uid.to_string());
    let followers = diesel::sql_query(query).load(conn).optional()?;

    Ok(followers)
}

// そのUserがフォローしてる相手を取得
pub fn find_followees_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
    // TODO: ちゃんとdieselでかく
    let query = format!("SELECT users.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join follows on follows.followee_id = users.id WHERE follows.allowed = true AND follows.follower_id = \'{}\';", uid.to_string());
    let followees = diesel::sql_query(query).load(conn)?;

    Ok(followees)
}
