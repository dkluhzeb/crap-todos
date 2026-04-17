//! Task CRUD server functions.

use leptos::prelude::*;

use crate::types::Tasks;

/// Fetch all tasks.
#[server]
pub async fn fetch_tasks() -> Result<Vec<Tasks>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindRequest {
        collection: "tasks".to_string(),
        limit: Some(100),
        depth: Some(1),
        draft: Some(true),
        ..Default::default()
    });

    let resp = client.find(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find: {e}")))?
        .into_inner();

    Ok(resp.documents.iter().map(Tasks::from_document).collect())
}

/// Fetch soft-deleted tasks (trash view).
#[server]
pub async fn fetch_trashed_tasks() -> Result<Vec<Tasks>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindRequest {
        collection: "tasks".to_string(),
        limit: Some(100),
        depth: Some(1),
        trash: Some(true),
        ..Default::default()
    });

    let resp = client.find(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find trashed: {e}")))?
        .into_inner();

    Ok(resp.documents.iter().map(Tasks::from_document).collect())
}

/// Fetch a single task by ID.
#[server]
pub async fn fetch_task(id: String) -> Result<Option<Tasks>, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(FindByIdRequest {
        collection: "tasks".to_string(),
        id,
        depth: Some(1),
        draft: Some(true),
        ..Default::default()
    });

    let resp = client.find_by_id(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC find_by_id: {e}")))?
        .into_inner();

    Ok(resp.document.as_ref().map(Tasks::from_document))
}

/// Create a new task.
#[server]
pub async fn create_task(title: String, status: String) -> Result<Tasks, ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(CreateRequest {
        collection: "tasks".to_string(),
        data: Some(build_struct(&[("title", Str(title)), ("status", Str(status)), ("priority", Num(1.0))])),
        locale: None,
        draft: None,
    });

    let doc = client.create(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC create: {e}")))?
        .into_inner()
        .document
        .ok_or_else(|| ServerFnError::new("No document in create response"))?;

    Ok(Tasks::from_document(&doc))
}

/// Update a task's fields.
#[server]
pub async fn update_task(
    id: String,
    title: Option<String>,
    status: Option<String>,
    priority: Option<f64>,
    due_date: Option<String>,
) -> Result<Tasks, ServerFnError> {
    use super::prelude::*;

    let mut entries: Vec<(&str, FieldValue)> = Vec::new();
    if let Some(v) = title { entries.push(("title", Str(v))); }
    if let Some(v) = status { entries.push(("status", Str(v))); }
    if let Some(v) = priority { entries.push(("priority", Num(v))); }
    if let Some(v) = due_date { entries.push(("due_date", Str(v))); }

    if entries.is_empty() {
        return Err(ServerFnError::new("No fields to update"));
    }

    let mut client = authed_client().await?;

    let req = Request::new(UpdateRequest {
        collection: "tasks".to_string(),
        id,
        data: Some(build_struct(&entries)),
        locale: None, draft: None, unpublish: None,
    });

    let doc = client.update(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC update: {e}")))?
        .into_inner()
        .document
        .ok_or_else(|| ServerFnError::new("No document in update response"))?;

    Ok(Tasks::from_document(&doc))
}

/// Update a task's status (for kanban column moves).
#[server]
pub async fn update_task_status(id: String, status: String) -> Result<(), ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(UpdateRequest {
        collection: "tasks".to_string(),
        id,
        data: Some(build_struct(&[("status", Str(status))])),
        locale: None, draft: None, unpublish: None,
    });

    client.update(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC update: {e}")))?;

    Ok(())
}

/// Update a task's assignee or category.
#[server]
pub async fn update_task_relation(
    id: String,
    assignee: Option<String>,
    category: Option<String>,
) -> Result<Tasks, ServerFnError> {
    use super::prelude::*;

    let mut entries = Vec::new();
    if let Some(v) = optional_relation(&assignee) { entries.push(("assignee", v)); }
    if let Some(v) = optional_relation(&category) { entries.push(("category", v)); }

    if entries.is_empty() {
        return Err(ServerFnError::new("No fields to update"));
    }

    let mut client = authed_client().await?;

    let req = Request::new(UpdateRequest {
        collection: "tasks".to_string(),
        id,
        data: Some(build_struct(&entries)),
        locale: None, draft: None, unpublish: None,
    });

    let doc = client.update(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC update: {e}")))?
        .into_inner()
        .document
        .ok_or_else(|| ServerFnError::new("No document in update response"))?;

    Ok(Tasks::from_document(&doc))
}

/// Soft-delete a task.
#[server]
pub async fn delete_task(id: String) -> Result<(), ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(DeleteRequest {
        collection: "tasks".to_string(),
        id,
        force_hard_delete: false,
    });

    client.delete(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC delete: {e}")))?;

    Ok(())
}

/// Permanently delete a task (hard delete).
#[server]
pub async fn hard_delete_task(id: String) -> Result<(), ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(DeleteRequest {
        collection: "tasks".to_string(),
        id,
        force_hard_delete: true,
    });

    client.delete(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC hard delete: {e}")))?;

    Ok(())
}

/// Restore a soft-deleted task from trash.
#[server]
pub async fn restore_task(id: String) -> Result<(), ServerFnError> {
    use super::prelude::*;

    let mut client = authed_client().await?;

    let req = Request::new(UndeleteRequest {
        collection: "tasks".to_string(),
        id,
    });

    client.undelete(auth_request(req).await?).await
        .map_err(|e| ServerFnError::new(format!("gRPC undelete: {e}")))?;

    Ok(())
}
