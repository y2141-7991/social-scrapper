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
        updated_at -> Nullable<Timestamp>,
        published_at -> Nullable<Timestamp>,
        name -> Nullable<Text>,
        avatar_url -> Text,
        biography -> Text,
        followers_count -> Nullable<Int8>,
        followings_count -> Nullable<Int8>,
        statuses_count -> Nullable<Int8>,
        link -> Text
    }
}