use social_store::store::Store;
use crate::crawl_account_by_username;


#[async_trait::async_trait]
pub trait SocialAccountService {
    async fn crawl_twitch_account_by_username(&self, username: &str) -> usize;
}

#[async_trait::async_trait]
impl SocialAccountService for Store {
    async fn crawl_twitch_account_by_username(&self, username: &str) -> usize {
        let mut conn = self.get_conn().await;
        let account = crawl_account_by_username(username).await.upsert(&mut conn).await.unwrap();
        account
    }
}

