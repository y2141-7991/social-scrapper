use tonic::{Request, Response, Status};
use social_grpc::social::api::social_account::v1::{social_account_api_service_server::SocialAccountApiService, SocialAccountRequest, SocialAccountReply};

use super::SocialAccountComponents;

#[tonic::async_trait]
impl SocialAccountApiService for SocialAccountComponents {
    async fn social_account(&self, request: Request<SocialAccountRequest>) -> Result<Response<SocialAccountReply>, Status> {
        let req = request.into_inner();

        // let response = self.store.find_social_account_by_social_account_id(account_id);

        Ok(Response::new(SocialAccountReply {
                social_name: String::from("value"),
                social_id: String::from("value"),
                username: String::from("value"),
                name: Some(String::from("value")),
                avatar_url: String::from("value"),
                biography: Some(String::from("value")),
                followers_count: Some(1),
                followings_count: Some(1),
                statuses_count: Some(1),
                link: String::from("value"),

        }))

    }
}