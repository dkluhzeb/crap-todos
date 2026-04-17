//! SSE bridge -- subscribes to crap-cms gRPC events and streams them to browsers.

use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::stream::Stream;
use grpc::{connect, proto::{MutationEvent, SubscribeRequest}};
use serde::Deserialize;
use serde_json::json;
use tonic::Request;
use tracing::{error, info, warn};

use crate::grpc;

#[derive(Deserialize)]
pub struct SseParams {
    token: Option<String>,
}

fn sse_mutation(event: &MutationEvent) -> Result<Event, Infallible> {
    let data = json!({
        "operation": event.operation,
        "collection": event.collection,
        "document_id": event.document_id,
        "sequence": event.sequence,
    });
    Ok(Event::default().event("mutation").data(data.to_string()))
}

/// SSE endpoint: GET /events?token=<jwt>
///
/// Subscribes to gRPC events for "tasks" and forwards them as SSE.
/// Returns 401 if the token is missing/invalid, which stops the browser's
/// EventSource from auto-reconnecting in a tight loop.
pub async fn events_handler(Query(params): Query<SseParams>) -> Response {
    let Some(ref token) = params.token else {
        warn!("SSE: no token provided");
        return (StatusCode::UNAUTHORIZED, "Token required").into_response();
    };

    let mut client = match connect().await {
        Ok(c) => c,
        Err(e) => {
            error!("SSE: gRPC connect failed: {e}");
            return (StatusCode::BAD_GATEWAY, format!("connect failed: {e}")).into_response();
        }
    };

    let mut req = Request::new(SubscribeRequest {
        collections: vec!["tasks".to_string()],
        ..Default::default()
    });
    let val = format!("Bearer {token}").parse().expect("valid header");
    req.metadata_mut().insert("authorization", val);

    let mut grpc_stream = match client.subscribe(req).await {
        Ok(resp) => resp.into_inner(),
        Err(e) => {
            error!("SSE: gRPC subscribe failed: {e}");
            return (StatusCode::UNAUTHORIZED, format!("subscribe failed: {e}")).into_response();
        }
    };

    info!("SSE: connected to gRPC subscribe stream");

    let event_stream = stream! {
        yield Ok::<_, Infallible>(Event::default().event("connected").data("ok"));

        loop {
            match grpc_stream.message().await {
                Ok(Some(event)) => yield sse_mutation(&event),
                Ok(None) => { info!("SSE: gRPC stream ended"); break; }
                Err(e) => { error!("SSE: gRPC stream error: {e}"); break; }
            }
        }
    };

    Sse::new(event_stream)
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("ping"))
        .into_response()
}
