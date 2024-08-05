use serde_json::{json, Value};
use reqwest::{Client, Error as APIError};

use crate::crawling::sessions::post_request;
use crate::crawling::gql::{PROFILE_GRAPHQL_FUNCTION, TWITCH_BASE_URL};

pub async fn get_user_by_login(login: &str) -> Result<Value, APIError> {
    let client = Client::new();
    let query = PROFILE_GRAPHQL_FUNCTION;
    let json = json!({
        "query": query,
        "variables": {
            "operationName": "User",
            "login": login
        }
    });
    let response = post_request(&client, TWITCH_BASE_URL, json);
    match response.await {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}