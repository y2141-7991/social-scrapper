use social_grpc::social::api::social_account::v1::{social_account_api_service_client::SocialAccountApiServiceClient, SocialAccountRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SocialAccountApiServiceClient::connect("http://[::1]:50051").await?;
    let req = SocialAccountRequest { social_name: "twitch".to_string(), social_id: "445529741".to_string()};
    let request = tonic::Request::new(req);
    let response = client.social_account(request).await;
    println!("{:?}", response);
    Ok(())
}
