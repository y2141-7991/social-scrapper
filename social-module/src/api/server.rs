use crate::api;
use social_store::store::Store;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;

pub async fn start_api_server(store: Store) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let (_, health_service) = tonic_health::server::health_reporter();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(social_grpc::_reflection::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(api::social_accounts::service(store.clone()))
        .serve(addr)
        .await?;

    Ok(())
}
