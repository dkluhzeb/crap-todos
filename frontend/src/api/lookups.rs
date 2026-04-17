//! Lookup server functions (users, categories).

use leptos::prelude::*;

use crate::types::{Categories, Users};

/// Fetch all users (for assignee picker).
#[server]
pub async fn fetch_users() -> Result<Vec<Users>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindRequest {
        collection: "users".to_string(),
        limit: Some(100),
        ..Default::default()
    });

    let resp = client.find(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find users: {e}")))?
        .into_inner();

    Ok(resp.documents.iter().map(Users::from_document).collect())
}

/// Fetch all categories (for category picker).
#[server]
pub async fn fetch_categories() -> Result<Vec<Categories>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindRequest {
        collection: "categories".to_string(),
        limit: Some(100),
        ..Default::default()
    });

    let resp = client.find(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find categories: {e}")))?
        .into_inner();

    Ok(resp.documents.iter().map(Categories::from_document).collect())
}
