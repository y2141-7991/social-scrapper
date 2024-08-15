use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable, Insertable, AsChangeset};
use crate::enums::SocialAccountEnum;


#[derive(Selectable, Insertable, Queryable, PartialEq, Debug, Identifiable)]
#[diesel(primary_key(social_name, social_id))]
#[diesel(table_name = crate::schema::social_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SocialAccount {
    pub social_name: String,
    pub social_id: String,
    pub username: String,
    pub status: SocialAccountEnum,
    pub updated_at: Option<NaiveDateTime>,
    pub published_at: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub avatar_url: String,
    pub biography: String,
    pub followers_count: Option<i32>,
    pub followings_count: Option<i32>,
    pub statuses_count: Option<i32>,
    pub link: String,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset)]
#[diesel(primary_key(social_name, social_id))]
#[diesel(table_name = crate::schema::social_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SocialAccountNew {
    pub social_name: String,
    pub social_id: String,
    pub username: String,
    pub status: SocialAccountEnum,
    pub published_at: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub avatar_url: String,
    pub biography: String,
    pub followers_count: Option<i32>,
    pub followings_count: Option<i32>,
    pub statuses_count: Option<i32>,
    pub link: String,
}