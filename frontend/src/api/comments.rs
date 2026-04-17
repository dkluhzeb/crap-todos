//! Comment server functions.

use leptos::prelude::*;

use crate::types::Comments;

/// Fetch comments for a task.
#[server]
pub async fn fetch_comments(task_id: String) -> Result<Vec<Comments>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindRequest {
        collection: "comments".to_string(),
        r#where: Some(format!(r#"{{"task": "{task_id}"}}"#)),
        limit: Some(50),
        depth: Some(1),
        order_by: Some("created_at".to_string()),
        ..Default::default()
    });

    let resp = client.find(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find comments: {e}")))?
        .into_inner();

    Ok(resp.documents.iter().map(Comments::from_document).collect())
}

/// Add a comment to a task.
#[server]
pub async fn create_comment(task_id: String, body: String) -> Result<(), ServerFnError> {
    use super::prelude::*;

    let token = extract_token().await?;

    let user_id = if let Some(ref t) = token {
        let mut client = authed_client().await?;
        let mut req = Request::new(MeRequest::default());
        let val = format!("Bearer {t}").parse().expect("valid header");
        req.metadata_mut().insert("authorization", val);
        client.me(req).await.ok().and_then(|r| r.into_inner().user).map(|u| u.id)
    } else {
        None
    };

    let Some(user_id) = user_id else {
        return Err(ServerFnError::new("Not authenticated"));
    };

    let mut client = authed_client().await?;

    let req = Request::new(CreateRequest {
        collection: "comments".to_string(),
        data: Some(build_struct(&[("body", Str(body)), ("task", Str(task_id)), ("author", Str(user_id))])),
        locale: None,
        draft: None,
    });

    client.create(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC create comment: {e}")))?;

    Ok(())
}
