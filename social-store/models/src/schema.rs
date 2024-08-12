diesel::table! {
    accounts {
        id -> Integer,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    social_account (social_name, social_id) {
        social_name -> Text,
        social_id -> Text,
        status -> Int4,
        username -> Text,
        updated_at -> Timestamp,
        published_at -> Timestamp,
        name -> Text,
        avatar_url -> Text,
        biography -> Text,
        followers_count -> Int8,
        followings_count -> Int8,
        statuses_count -> Int8,
        link -> Text
    }
}