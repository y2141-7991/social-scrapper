use crate::store::Store;
use models::social_account::SocialAccount;

#[async_trait::async_trait]
pub trait SocialAccountInterface {
    async fn find_social_account_by_social_account_id(
        &self,
        account_id: (String, String),
    ) -> Vec<SocialAccount>;
}

#[async_trait::async_trait]
impl SocialAccountInterface for Store {
    async fn find_social_account_by_social_account_id(
        &self,
        account_id: (String, String),
    ) -> Vec<SocialAccount> {
        let mut conn = self.get_conn().await;
        let social_account =
            SocialAccount::find_social_profile_by_social_account_id(&mut conn, account_id).await;
        social_account
    }
}
