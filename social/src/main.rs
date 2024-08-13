use crawlers::crawl_account_by_username;
use reqwest::Error as ReqwestError;
use social_store::store::Store;
use models::social_account::SocialAccount;


mod crawling;
mod crawlers;
mod infra;
mod utils;


#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());
    let mut conn = store.get_conn().await;

    let account = SocialAccount::find_social_profile_by_social_account_id(&mut conn, ("twitter".to_string(), "12345678901234567".to_string())).await;
    // let user = crawl_account_by_username("tarik").await;
    // println!("{:?}", user);
    // Ok(())
}
