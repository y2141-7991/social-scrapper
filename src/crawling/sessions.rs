use reqwest::{header::{self, HeaderMap}, Client};
use serde_json::Value;

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

pub async fn post_request(client: &Client, url: &str, json: serde_json::Value) -> Result<Value, reqwest::Error> {
    let headers = prepare_headers();
    let response = client.post(url).headers(headers).json(&json).send().await?;

    if !response.status().is_success() {
        panic!()
    }
    match response.json::<Value>().await {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}
