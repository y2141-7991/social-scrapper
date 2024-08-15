use crawlers::crawl_account_by_username;
use models::social_account::SocialAccount;
use services::SocialAccountService;
use social_store::store::Store;

mod crawlers;
mod crawling;
mod infra;
mod utils;
mod services;

#[tokio::main]
async fn main() {
    let service = Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());
    // let account = SocialAccount::find_social_profile_by_social_account_id(
    //     &mut conn,
    //     ("twitch".to_string(), "36340781".to_string()),
    // )
    // .await;

    let user = service.crawl_twitch_account_by_username("olofmeister")
        .await;
    println!("{:?}", user);
}
