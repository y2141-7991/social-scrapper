use crawlers::crawl_account_by_username;
use reqwest::Error as ReqwestError;


mod crawling;
mod crawlers;
mod infra;
mod utils;


#[tokio::main]
async fn main() -> Result<(), ReqwestError>{
    let user = crawl_account_by_username("tarik").await;
    println!("{:?}", user);
    Ok(())
}
