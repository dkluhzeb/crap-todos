//! SSE bridge -- subscribes to crap-cms gRPC events and streams them to browsers.

use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use grpc::{
    connect,
    proto::{MutationEvent, SubscribeRequest},
};
use serde::Deserialize;
use serde_json::json;
use tonic::Request;
use tracing::{error, info};

use crate::grpc;

#[derive(Deserialize)]
pub struct SseParams {
    token: Option<String>,
}

fn sse_error(msg: String) -> Result<Event, Infallible> {
    Ok(Event::default().event("error").data(msg))
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
pub async fn events_handler(
    Query(params): Query<SseParams>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream! {
        let mut client = match connect().await {
            Ok(c) => c,
            Err(e) => {
                error!("SSE: gRPC connect failed: {e}");
                yield sse_error(format!("connect failed: {e}"));
                return;
            }
        };

        let mut req = Request::new(SubscribeRequest {
            collections: vec!["tasks".to_string()],
            ..Default::default()
        });

        if let Some(ref token) = params.token {
            let val = format!("Bearer {token}").parse().expect("valid header");
            req.metadata_mut().insert("authorization", val);
        }

        let mut stream = match client.subscribe(req).await {
            Ok(resp) => resp.into_inner(),
            Err(e) => {
                error!("SSE: gRPC subscribe failed: {e}");
                yield sse_error(format!("subscribe failed: {e}"));
                return;
            }
        };

        info!("SSE: connected to gRPC subscribe stream");
        yield Ok(Event::default().event("connected").data("ok"));

        loop {
            match stream.message().await {
                Ok(Some(event)) => yield sse_mutation(&event),
                Ok(None) => { info!("SSE: gRPC stream ended"); break; }
                Err(e) => { error!("SSE: gRPC stream error: {e}"); yield sse_error(format!("{e}")); break; }
            }
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
