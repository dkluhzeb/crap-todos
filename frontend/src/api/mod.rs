//! Server functions -- gRPC calls exposed to Leptos components.

mod auth;
mod comments;
#[cfg(feature = "ssr")]
mod helpers;
mod lookups;
mod tasks;

/// Prelude for `#[server]` functions -- single import for all SSR helpers.
///
/// ```ignore
/// use super::prelude::*;
/// ```
#[cfg(feature = "ssr")]
pub(crate) mod prelude {
    pub use crate::generated_proto::FromDocument;
    pub use crate::grpc;
    pub use crate::grpc::proto::{
        CreateRequest, DeleteRequest, FindByIdRequest, FindRequest, LoginRequest, MeRequest,
        UndeleteRequest, UpdateRequest,
    };
    pub use http::header::SET_COOKIE;
    pub use leptos_axum::ResponseOptions;
    pub use tonic::Request;

    pub use super::helpers::{
        FieldValue::{self, *},
        auth_request, authed_client, build_struct, extract_token, optional_relation,
    };
}

pub use auth::*;
pub use comments::*;
pub use lookups::*;
pub use tasks::*;
