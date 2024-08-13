use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable, Insertable};
use crate::enums::SocialAccountStatus;


#[derive(Selectable, Insertable, Queryable, PartialEq, Debug, Identifiable)]
#[diesel(primary_key(social_name, social_id))]
#[diesel(table_name = crate::schema::social_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SocialAccount {
    pub social_name: String,
    pub social_id: String,
    pub username: String,
    pub status: SocialAccountStatus,
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

