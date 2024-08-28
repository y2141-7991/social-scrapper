use social_store::store::Store;
use social_grpc::social::api::social_account::v1;

pub mod service;

#[derive(Clone)]
pub struct SocialAccountComponents {
    store: Store
}


// pub fn service(store: Store) -> SocialAccountApiServiceServer<SocialAccountComponents> {
//     let inner = SocialAccountComponents { store };
//     let a = SocialAccountApiServiceServer::new(inner);
//     a
// }