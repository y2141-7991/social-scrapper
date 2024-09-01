use social_grpc::social::api::social_account::v1::{
    social_account_api_service_server::SocialAccountApiService, ListSocialAccountResponse,
    SocialAccountRequest, SocialAccountResponse,
};
use social_store::repositories::social_account::SocialAccountInterface;
use tonic::{Request, Response, Status};

use super::SocialAccountComponents;

#[tonic::async_trait]
impl SocialAccountApiService for SocialAccountComponents {
    async fn social_account(
        &self,
        request: Request<SocialAccountRequest>,
    ) -> Result<Response<ListSocialAccountResponse>, Status> {
        let req = request.into_inner();
        let response = self
            .store
            .find_social_account_by_social_account_id((req.social_name, req.social_id))
            .await;
        let mut response_social_accounts: Vec<SocialAccountResponse> = Vec::new();
        for sa in response {
            response_social_accounts.push(SocialAccountResponse {
                social_name: sa.social_name,
                social_id: sa.social_id,
                username: sa.username,
                name: sa.name,
                avatar_url: sa.avatar_url,
                biography: sa.biography,
                followers_count: sa.followers_count,
                followings_count: sa.followings_count,
                statuses_count: sa.statuses_count,
                link: sa.link,
            });
        }
        Ok(Response::new(ListSocialAccountResponse {
            social_accounts: response_social_accounts,
        }))
    }
}
