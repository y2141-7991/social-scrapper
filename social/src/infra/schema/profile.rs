use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct SocialProfile {
    pub social_name: String,
    pub social_id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub avatar_url: String,
    pub biography: String,
    pub followers_count: i128,
    pub followings_count: i128,
}
