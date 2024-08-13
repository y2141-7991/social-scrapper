use models::social_account::SocialAccount;
use social_store::store::Store;


#[tokio::main]
async fn main() {
    let store = Store::new("database_url".to_string());
    let mut conn = store.get_conn().await;
    let _ = SocialAccount::find_social_profile_by_social_account_id(&mut conn, ("tw".to_string(), "".to_string())).await;
}