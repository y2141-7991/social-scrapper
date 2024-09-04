

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = social_store::store::Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());
    let private_server = social_module::api::server::start_api_server(store);
    tokio::select! {
        _ = private_server => {}
    }
    Ok(())
}
