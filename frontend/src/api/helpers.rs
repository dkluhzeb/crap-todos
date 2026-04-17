//! Server-side helpers for gRPC communication.
//!
//! Import everything via `use super::helpers::prelude::*;` inside `#[server]` functions.

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::{grpc, types::AuthToken};
#[cfg(feature = "ssr")]
use grpc::proto;
#[cfg(feature = "ssr")]
use http::header::COOKIE;
#[cfg(feature = "ssr")]
use prost_types::{Struct, Value, value::Kind};
#[cfg(feature = "ssr")]
use std::collections::BTreeMap;
#[cfg(feature = "ssr")]
use tonic::Request;

/// gRPC client type alias.
#[cfg(feature = "ssr")]
type GrpcClient = proto::content_api_client::ContentApiClient<tonic::transport::Channel>;


#[cfg(feature = "ssr")]
pub async fn extract_token() -> Result<Option<String>, ServerFnError> {
    if let Some(auth) = use_context::<AuthToken>()
        && auth.0.is_some()
    {
        return Ok(auth.0);
    }

    let headers: http::HeaderMap = match leptos_axum::extract().await {
        Ok(h) => h,
        Err(_) => return Ok(None),
    };

    let token = headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|s| s.split(';'))
        .map(|s| s.trim())
        .find(|s| s.starts_with("crap_token="))
        .map(|s| s.trim_start_matches("crap_token=").to_string());

    Ok(token)
}

#[cfg(feature = "ssr")]
pub async fn authed_client() -> Result<GrpcClient, ServerFnError> {
    grpc::connect()
        .await
        .map_err(|e| ServerFnError::new(format!("gRPC connect: {e}")))
}

#[cfg(feature = "ssr")]
pub async fn auth_request<T>(mut req: Request<T>) -> Result<Request<T>, ServerFnError> {
    if let Some(token) = extract_token().await? {
        let val = format!("Bearer {token}").parse().expect("valid header");
        req.metadata_mut().insert("authorization", val);
    }

    Ok(req)
}

/// Build a prost `Struct` from key-value pairs for gRPC requests.
#[cfg(feature = "ssr")]
pub fn build_struct(entries: &[(&str, FieldValue)]) -> Struct {
    let mut fields = BTreeMap::new();

    for (key, val) in entries {
        let kind = match val {
            FieldValue::Str(s) => Some(Kind::StringValue(s.clone())),
            FieldValue::Num(n) => Some(Kind::NumberValue(*n)),
            FieldValue::Null => Some(Kind::NullValue(0)),
        };
        fields.insert(key.to_string(), Value { kind });
    }

    Struct { fields }
}

/// Build optional string/null field for relation updates.
#[cfg(feature = "ssr")]
pub fn optional_relation(value: &Option<String>) -> Option<FieldValue> {
    value.as_ref().map(|v| {
        if v.is_empty() {
            FieldValue::Null
        } else {
            FieldValue::Str(v.clone())
        }
    })
}

#[cfg(feature = "ssr")]
pub enum FieldValue {
    Str(String),
    Num(f64),
    Null,
}
