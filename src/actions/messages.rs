use diesel::prelude::*;
use uuid::Uuid;

use super::super::models;

pub fn find_message_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Message>, diesel::result::Error> {
    use crate::schema::messages::dsl::*;

    let _message = messages
        .filter(id.eq(uid.to_string()))
        .first::<models::Message>(conn)
        .optional()?;

    Ok(_message)
}

pub fn find_messages(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Message>>, diesel::result::Error> {
    use crate::schema::messages::dsl::*;

    let _message = messages
        .load::<models::Message>(conn)
        .optional()?;

    Ok(_message)
}

pub fn insert_new_message(
    new_message: models::Message,
    conn: &PgConnection,
) -> Result<models::Message, diesel::result::Error> {
    use crate::schema::messages::dsl::*;

    diesel::insert_into(messages).values(&new_message).execute(conn)?;

    Ok(new_message)
}

pub fn find_messages_by_to_user_id(
    user_id: String,
    conn: &PgConnection,
) -> Result<Option<models::Message>, diesel::result::Error> {
    use crate::schema::messages::dsl::*;

    let _message = messages
        .filter(to_user_id.eq(user_id))
        .first::<models::Message>(conn)
        .optional()?;

    Ok(_message)
}