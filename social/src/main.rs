use crawlers::crawl_account_by_username;
use futures::StreamExt;
use services::*;
use social_store::store::Store;

mod crawlers;
mod crawling;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // let store = Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());

    // use models::social_account::SocialAccount;
    // let mut conn = store.get_conn().await;
    // let account = SocialAccount::find_social_profile_by_social_account_id(
    //     &mut conn,
    //     ("twitch".to_string(), "36340781".to_string()),
    // )
    // .await;
    // println!("{:?}", account);

    // let service1 = SocialAccountCrawlingComponents{ store };
    // let acc = service1.crawl_twitch_account_by_username("tarik").await;
    // println!("{:?}", acc);


    // test_tokio_spawn().await

    for i in 0..5 {
        test_rand().await
    }
}


async fn test_rand() {
    use rand::Rng;
    use std::thread;
    use std::time::Duration;

    let random_number: u32 = rand::thread_rng().gen_range(0, 10);
    println!("Random number between 1 and 10: {}", random_number);
    thread::sleep(Duration::from_secs(1));
}

async fn test_tokio_spawn() { 
    use futures::stream::FuturesOrdered;
    (0..5).map(|_| {
        tokio::spawn(test_rand())
    }).collect::<FuturesOrdered<_>>().for_each(|_| async {}).await
}