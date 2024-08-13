use crate::store::Store;
use models::social_account::SocialAccount;
use diesel::query_dsl::methods::{FilterDsl};
use diesel::{ExpressionMethods};
use diesel_async::{RunQueryDsl};

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