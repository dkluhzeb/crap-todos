//! gRPC client wrapper for crap-cms.
//!
//! Uses a lazily-initialized shared channel so all server functions
//! multiplex over a single HTTP/2 connection instead of opening a
//! new TCP connection per request.

pub mod proto {
    tonic::include_proto!("crap");
}

use std::env;

use proto::content_api_client::ContentApiClient;
use tokio::sync::OnceCell;
use tonic::transport::{Channel, Error};

static CHANNEL: OnceCell<Channel> = OnceCell::const_new();

async fn shared_channel() -> Result<Channel, Error> {
    CHANNEL
        .get_or_try_init(|| async {
            let url = env::var("GRPC_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());
            Channel::from_shared(url)
                .expect("valid URL")
                .connect()
                .await
        })
        .await
        .cloned()
}

/// Get a gRPC client using the shared channel.
pub async fn connect() -> Result<ContentApiClient<Channel>, Error> {
    let channel = shared_channel().await?;
    Ok(ContentApiClient::new(channel))
}
