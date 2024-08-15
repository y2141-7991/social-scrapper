use crawlers::crawl_account_by_username;
use reqwest::Error as ReqwestError;
use social_store::store::Store;
use models::social_account::{SocialAccount, SocialAccountNew};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};


mod crawling;
mod crawlers;
mod infra;
mod utils;


#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:123@localhost:5432/postgres".to_string());
    let mut conn = store.get_conn().await;
    let account = SocialAccount::find_social_profile_by_social_account_id(&mut conn, ("twitch".to_string(), "36340781".to_string())).await;

    let user = crawl_account_by_username("tarik").await.upsert(&mut conn).await;
    println!("{:?}", user);
}

