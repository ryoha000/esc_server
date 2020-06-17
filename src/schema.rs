table! {
    brands (id) {
        id -> Int4,
        brandname -> Text,
        brandfurigana -> Nullable<Text>,
        makername -> Nullable<Text>,
        makerfurigana -> Nullable<Text>,
        url -> Nullable<Text>,
        checked -> Nullable<Bool>,
        kind -> Nullable<Text>,
        lost -> Nullable<Bool>,
        directlink -> Nullable<Bool>,
        median -> Nullable<Int4>,
        http_response_code -> Nullable<Int4>,
        twitter -> Nullable<Text>,
        twitter_data_widget_id -> Nullable<Int4>,
        notes -> Nullable<Text>,
        erogetrailers -> Nullable<Int4>,
        cien -> Nullable<Int4>,
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
    brands,
    users,
);
