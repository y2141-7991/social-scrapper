use social_store::store::Store;
use crate::crawl_account_by_username;


#[derive(Clone)]
pub struct SocialAccountCrawlingComponents {
    pub store: Store,
}

#[async_trait::async_trait]
pub trait SocialAccountService {
    async fn crawl_twitch_account_by_username(&self, username: &str) -> usize;
}

#[async_trait::async_trait]
impl SocialAccountService for SocialAccountCrawlingComponents {
    async fn crawl_twitch_account_by_username(&self, username: &str) -> usize {
        let mut conn = self.store.get_conn().await;
        let account = crawl_account_by_username(username).await.upsert(&mut conn).await.unwrap();
        account
    }
}