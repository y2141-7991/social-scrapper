use crate::crawling::apis::get_user_by_login;
use crate::utils::convert_string_to_datetime;
use models::social_account::SocialAccountNew;
use std::fmt::Write;

fn generate_profile_url(username: String) -> String {
    let mut url = String::from("https://www.twitch.tv/");

    let username_trimmed = username.trim();

    url.write_fmt(format_args!("/{}", username_trimmed))
        .expect("Can not write this format");

    url
}

pub async fn crawl_account_by_username(login: &str) -> SocialAccountNew {
    let user_response = get_user_by_login(login).await.unwrap();
    let sa: SocialAccountNew = SocialAccountNew {
        social_name: "twitch".to_string(),
        social_id: user_response["data"]["user"]["id"]
            .as_str()
            .unwrap()
            .to_string(),
        status: models::enums::SocialAccountEnum::Active,
        username: user_response["data"]["user"]["displayName"]
            .as_str()
            .unwrap()
            .to_string(),
        name: Some(
            user_response["data"]["user"]["displayName"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        link: generate_profile_url(
            user_response["data"]["user"]["displayName"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        published_at: Some(convert_string_to_datetime(
            user_response["data"]["user"]["createdAt"].as_str().unwrap(),
        )),
        avatar_url: user_response["data"]["user"]["profileImageURL"]
            .as_str()
            .unwrap()
            .to_string(),
        biography: user_response["data"]["user"]["description"]
            .as_str()
            .unwrap()
            .to_string(),
        followers_count: Some(
            user_response["data"]["user"]["followers"]["totalCount"]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        ),
        followings_count: Some(0),
        statuses_count: Some(0),
    };
    sa
}
