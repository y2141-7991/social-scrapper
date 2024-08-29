use crawlers::crawl_account_by_username;
use services::*;
use social_store::store::Store;

mod crawlers;
mod crawling;
mod utils;
mod services;

#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());

    use models::social_account::SocialAccount;
    let mut conn = store.get_conn().await;
    let account = SocialAccount::find_social_profile_by_social_account_id(
        &mut conn,
        ("twitch".to_string(), "36340781".to_string()),
    )
    .await;
    println!("{:?}", account);

    // let service1 = SocialAccountCrawlingComponents{ store };
    // let acc = service1.crawl_twitch_account_by_username("tarik").await;
    // println!("{:?}", acc);
}
