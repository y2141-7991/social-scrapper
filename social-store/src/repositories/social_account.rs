use crate::store::Store;

#[async_trait::async_trait]
pub trait SocialAccountInterface {
    async fn find_social_account_by_social_account_id(&self, account_id: (String, String));
}

#[async_trait::async_trait]
impl SocialAccountInterface for Store {
    async fn find_social_account_by_social_account_id(&self, account_id: (String, String)) {
        let mut conn = self.get_conn().await;
    }
}
