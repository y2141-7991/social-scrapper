use social_grpc::social::api::social_account::v1::social_account_api_service_server::SocialAccountApiServiceServer;
use social_store::store::Store;

pub mod service;

#[derive(Clone)]
pub struct SocialAccountComponents {
    store: Store,
}

pub fn service(store: Store) -> SocialAccountApiServiceServer<SocialAccountComponents> {
    let inner = SocialAccountComponents { store };
    SocialAccountApiServiceServer::new(inner)
}
