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
    follower_id: String,
    conn: &PgConnection,
) -> Result<Option<Vec<models::User>>, diesel::result::Error> {
    // TODO: ちゃんとdieselでかく
    let query = format!("SELECT users.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join follows on follows.follower_id = users.id WHERE follows.allowed = true AND follows.followee_id = \'{}\';", follower_id);
    let followers = diesel::sql_query(query).load(conn).optional()?;

    Ok(followers)
}

// そのUserがフォローしてる相手を取得
pub fn find_followees_by_uid(
    followee_id: String,
    conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
    // TODO: ちゃんとdieselでかく
    let query = format!("SELECT users.id, users.es_user_id, users.display_name, users.comment, users.show_all_users, users.show_detail_all_users, users.show_followers, users.show_followers_okazu, users.twitter_id from users inner join follows on follows.followee_id = users.id WHERE follows.allowed = true AND follows.follower_id = \'{}\';", followee_id);
    let followees = diesel::sql_query(query).load(conn)?;

    Ok(followees)
}

pub fn delete_follow(
    follow_id: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Vec<models::Follow>, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    let deleted_follow = diesel::update(follows.filter(id.eq(follow_id.to_string())))
        .set(deleted_at.eq(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0)))
        .load(conn)?;

    Ok(deleted_follow)
}

pub fn approve_follow(
    follow_id: uuid::Uuid,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    let approve_follows: Vec<models::Follow> = diesel::update(follows.filter(id.eq(follow_id.to_string())))
        .set(allowed.eq(true))
        .load(conn)?;

    if approve_follows.len() > 0 {
        let approve_follow = &approve_follows[0];
        if approve_follow.mutual {
            let mut follow_back = models::Follow::new(approve_follow.follower_id.clone(), approve_follow.followee_id.clone());
            follow_back.mutual = true;
            follow_back.allowed = true;
            let follow_back = insert_new_follow(follow_back, conn)?;
        }
    }
    Ok(())
}
