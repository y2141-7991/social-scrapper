pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "SocialAccountEnum"))]
    pub struct SocialAccountEnum;
}

diesel::table! {
    accounts {
        id -> Integer,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SocialAccountEnum;

    social_account (social_name, social_id) {
        social_name -> Text,
        social_id -> Text,
        status -> SocialAccountEnum,
        username -> Text,
        updated_at -> Nullable<Timestamp>,
        published_at -> Nullable<Timestamp>,
        name -> Nullable<Text>,
        avatar_url -> Text,
        biography -> Text,
        followers_count -> Nullable<Integer>,
        followings_count -> Nullable<Integer>,
        statuses_count -> Nullable<Integer>,
        link -> Text
    }
}
