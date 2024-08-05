use reqwest::{header::{self, HeaderMap}, Client};
use reqwest::Error as ReqwestError;
use serde_json::{json, Value};
use chrono::{DateTime, Utc};

fn prepare_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Connection", "Keep-Alive".parse().unwrap());
    headers.insert("Client-ID", "ue6666qo983tsx6so1t0vnawi233wa".parse().unwrap());
    headers.insert("Accept-Language", "en-us".parse().unwrap());
    headers.insert("Api-Consumer-Type", "tv; androidtv_web_tv/sst-1b53dfa".parse().unwrap());
    headers.insert("X-App-Version", "16.8.1".parse().unwrap());
    headers.insert("User-Agent", "com.zhiliaoapp.musically/2018090613 (Linux; U; Android 8.0.0; tr_TR; TA-1020; Build/O00623; Cronet/58.0.2991.0)".parse().unwrap());
    headers
}

async fn call_api() -> Result<Value, ReqwestError> {
    let data = json!({
        "query": "
        query ChannelHome_Query($login: String!) {
        user(login: $login) {
        ...ProfileBanner_channel

        }
    }


    fragment ProfileBanner_channel on User {
        roles {
            isAffiliate
            isGlobalMod 
            isPartner 
            isSiteAdmin 
            isStaff
        }
        createdAt
        displayName
        id
        login
        profileImageURL(width: 300)
        followers { totalCount }
        description
        channel {
            __typename
            socialMedias 
            {
                __typename
                id
                name
                title
                url
            }
        }

    }

        ",
        "variables": {
            "operationName": "User",
            "login": "tarik"
        }
    });
    
    let client = Client::new();
    let headers = prepare_headers();
    let response = client.post("https://gql.twitch.tv/gql").headers(headers).json(&data).send().await?;

    match response.json::<Value>().await {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}

fn convert_string_to_datetime(datetime_string: &str) -> DateTime<Utc> {
    let datetime = DateTime::parse_from_rfc3339(&datetime_string).unwrap().into();
    println!("{}", datetime);
    datetime
}


#[derive(Debug)]
struct SocialProfile {
    social_name: String,
    social_id: String,
    username: String,
    created_at: DateTime<Utc>,
    avatar_url: String,
    biography: String,
    followers_count: i128,
    followings_count: i128,
}


#[tokio::main]
async fn main() -> Result<(), ReqwestError>{
    let res = call_api().await?;

    let sp = SocialProfile {
        social_name: "twitch".to_string(),
        social_id: res["data"]["user"]["id"].to_string(),
        username: res["data"]["user"]["displayName"].to_string(),
        created_at: convert_string_to_datetime(res["data"]["user"]["createdAt"].as_str().unwrap()),
        avatar_url: res["data"]["user"]["profileImageURL"].to_string(),
        biography: res["data"]["user"]["description"].to_string(),
        followers_count: res["data"]["user"]["followers"]["totalCount"].to_string().parse::<i128>().unwrap(),
        followings_count: 0
    };
    println!("{:#?}", sp);
    Ok(())
}
