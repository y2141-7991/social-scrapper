use social_store::store::Store;

pub mod service;

#[derive(Clone)]
pub struct SocialAccountComponents {
    store: Store
}


pub fn service(store: Store) {

}