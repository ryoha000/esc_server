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
        scheduled_date -> Date,
    }
}

table! {
    games (id) {
        id -> Int4,
        gamename -> Nullable<Text>,
        furigana -> Nullable<Text>,
        sellday -> Nullable<Date>,
        brand_id -> Int4,
        comike -> Nullable<Int4>,
        shoukai -> Nullable<Text>,
        model -> Nullable<Text>,
        erogame -> Nullable<Bool>,
        banner_url -> Nullable<Text>,
        gyutto_id -> Nullable<Int4>,
        dmm -> Nullable<Text>,
        dmm_genre -> Nullable<Text>,
        dmm_genre_2 -> Nullable<Text>,
        erogametokuten -> Nullable<Int4>,
        total_play_time_median -> Nullable<Int4>,
        time_before_understanding_fun_median -> Nullable<Int4>,
        dlsite_id -> Nullable<Text>,
        dlsite_domain -> Nullable<Text>,
        trial_url -> Nullable<Text>,
        okazu -> Nullable<Bool>,
        axis_of_soft_or_hard -> Nullable<Int4>,
        genre -> Nullable<Text>,
        twitter -> Nullable<Text>,
        digiket -> Nullable<Text>,
        twitter_data_widget_id -> Nullable<Int4>,
        masterup -> Nullable<Date>,
        steam -> Nullable<Int4>,
        dlsite_rental -> Nullable<Bool>,
        dmm_subsc -> Nullable<Text>,
        surugaya_1 -> Nullable<Int4>,
        scheduled_date -> Date,
    }
}

table! {
    timelines (id) {
        id -> Varchar,
        user_id -> Varchar,
        game_id -> Int4,
        log_type -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        es_user_id -> Varchar,
        display_name -> Varchar,
        comment -> Nullable<Text>,
        show_all_users -> Nullable<Bool>,
        show_detail_all_users -> Nullable<Bool>,
        show_followers -> Nullable<Bool>,
        show_followers_okazu -> Nullable<Bool>,
        twitter_id -> Nullable<Varchar>,
    }
}

joinable!(games -> brands (brand_id));
joinable!(timelines -> games (game_id));
joinable!(timelines -> users (user_id));

allow_tables_to_appear_in_same_query!(
    brands,
    games,
    timelines,
    users,
);
