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
    follows (id) {
        id -> Varchar,
        followee_id -> Varchar,
        follower_id -> Varchar,
        allowed -> Bool,
        mutual -> Bool,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
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
    listlogs (id) {
        id -> Varchar,
        timeline_id -> Varchar,
        list_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    listmaps (id) {
        id -> Varchar,
        list_id -> Varchar,
        game_id -> Int4,
    }
}

table! {
    lists (id) {
        id -> Varchar,
        user_id -> Varchar,
        name -> Text,
        comment -> Text,
        priority -> Int4,
        url -> Nullable<Text>,
        is_public -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Varchar,
        from_user_id -> Varchar,
        to_user_id -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    reviewlogs (id) {
        id -> Varchar,
        timeline_id -> Varchar,
        review_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    reviews (id) {
        id -> Varchar,
        game_id -> Int4,
        user_id -> Varchar,
        es_user_id -> Text,
        tokuten -> Nullable<Int4>,
        tourokubi -> Nullable<Timestamp>,
        hitokoto -> Nullable<Text>,
        memo -> Nullable<Text>,
        netabare -> Nullable<Bool>,
        giveup -> Nullable<Bool>,
        possession -> Nullable<Bool>,
        play -> Nullable<Bool>,
        before_hitokoto -> Nullable<Text>,
        before_tokuten -> Nullable<Int4>,
        before_tourokubi -> Nullable<Timestamp>,
        display -> Nullable<Bool>,
        play_tourokubi -> Nullable<Timestamp>,
        display_unique_count -> Nullable<Int4>,
        sage -> Nullable<Bool>,
        before_purchase_will -> Nullable<Text>,
        before_sage -> Nullable<Bool>,
        total_play_time -> Nullable<Int4>,
        time_before_understanding_fun -> Nullable<Int4>,
        okazu_tokuten -> Nullable<Int4>,
        trial_version_hitokoto -> Nullable<Text>,
        trial_version_hitokoto_sage -> Nullable<Bool>,
        trial_version_hitokoto_tourokubi -> Nullable<Timestamp>,
        es_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
joinable!(listlogs -> lists (list_id));
joinable!(listlogs -> timelines (timeline_id));
joinable!(listmaps -> games (game_id));
joinable!(listmaps -> lists (list_id));
joinable!(lists -> users (user_id));
joinable!(reviewlogs -> reviews (review_id));
joinable!(reviewlogs -> timelines (timeline_id));
joinable!(reviews -> games (game_id));
joinable!(reviews -> users (user_id));
joinable!(timelines -> games (game_id));
joinable!(timelines -> users (user_id));

allow_tables_to_appear_in_same_query!(
    brands,
    follows,
    games,
    listlogs,
    listmaps,
    lists,
    messages,
    reviewlogs,
    reviews,
    timelines,
    users,
);
