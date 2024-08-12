use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{Identifiable, Queryable, Selectable, Insertable};
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Selectable, Insertable, Identifiable, PartialEq, Debug)]
#[diesel(primary_key(social_name, social_id))]
#[diesel(table_name = crate::schema::social_account)]
pub struct SocialAccount {
    pub social_name: String,
    pub social_id: String,
    pub username: String,
    pub status: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub published_at: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub avatar_url: String,
    pub biography: String,
    pub followers_count: Option<i64>,
    pub followings_count: Option<i64>,
    pub statuses_count: Option<i64>,
    pub link: String,
}

