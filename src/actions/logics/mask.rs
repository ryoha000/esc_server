use super::super::super::models;
use super::super::follows;
use diesel::prelude::*;
use uuid::Uuid;

pub fn mask(
    user_id: Uuid,
    timelines: &Vec<models::Timeline>,
    conn: &PgConnection,
) {
    let followees = follows::find_followees_by_uid(user_id, conn);

}
