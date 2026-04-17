//! gRPC client wrapper for crap-cms.

pub mod proto {
    tonic::include_proto!("crap");
}

use std::env;

use proto::content_api_client::ContentApiClient;
use tonic::transport::{Channel, Error};

/// Create a gRPC client connected to crap-cms.
pub async fn connect() -> Result<ContentApiClient<Channel>, Error> {
    let url = env::var("GRPC_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

    ContentApiClient::connect(url).await
}
