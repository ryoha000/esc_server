table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        display_name -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
