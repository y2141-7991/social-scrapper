use crate::crawling::apis::get_user_by_login;
use crate::infra::schema::profile::SocialProfile;
use crate::utils::convert_string_to_datetime;

pub async fn crawl_account_by_username(login: &str) -> SocialProfile {
    let user_response = get_user_by_login(login).await.unwrap();
    let sp = SocialProfile {
        social_name: "twitch".to_string(),
        social_id: user_response["data"]["user"]["id"].to_string(),
        username: user_response["data"]["user"]["displayName"].to_string(),
        created_at: convert_string_to_datetime(user_response["data"]["user"]["createdAt"].as_str().unwrap()),
        avatar_url: user_response["data"]["user"]["profileImageURL"].to_string(),
        biography: user_response["data"]["user"]["description"].to_string(),
        followers_count: user_response["data"]["user"]["followers"]["totalCount"].to_string().parse::<i128>().unwrap(),
        followings_count: 0
    };
    sp
}