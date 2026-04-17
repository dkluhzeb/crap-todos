//! Authentication server functions.

use leptos::prelude::*;

use crate::types::Users;

/// Login and return user info. Sets JWT cookie on success.
#[server]
pub async fn login(email: String, password: String) -> Result<Users, ServerFnError> {
    use super::prelude::*;

    let mut client = grpc::connect().await
        .map_err(|e| ServerFnError::new(format!("gRPC connect: {e}")))?;

    let resp = client
        .login(Request::new(LoginRequest {
            collection: "users".to_string(),
            email,
            password,
        }))
        .await
        .map_err(|e| ServerFnError::new(format!("Login failed: {e}")))?
        .into_inner();

    let user_doc = resp.user
        .ok_or_else(|| ServerFnError::new("No user in login response"))?;

    let response_opts = expect_context::<ResponseOptions>();
    let cookie = format!("crap_token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=86400", resp.token);
    response_opts.insert_header(SET_COOKIE, cookie.parse().unwrap());

    Ok(Users::from_document(&user_doc))
}

/// Logout -- clear the JWT cookie.
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use super::prelude::*;

    let response_opts = expect_context::<ResponseOptions>();
    response_opts.insert_header(SET_COOKIE, "crap_token=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".parse().unwrap());

    Ok(())
}

/// Get the current user from the JWT cookie (if valid).
#[server]
pub async fn get_current_user() -> Result<Option<Users>, ServerFnError> {
    use super::prelude::*;

    let Some(token) = extract_token().await? else {
        return Ok(None);
    };

    let mut client = grpc::connect().await
        .map_err(|e| ServerFnError::new(format!("gRPC connect: {e}")))?;

    let mut req = Request::new(MeRequest::default());
    req.metadata_mut().insert("authorization", format!("Bearer {token}").parse().expect("valid header"));

    match client.me(req).await {
        Ok(resp) => Ok(resp.into_inner().user.map(|d| Users::from_document(&d))),
        Err(_) => Ok(None),
    }
}

/// Get the JWT token for SSE connection.
#[server]
pub async fn get_sse_token() -> Result<Option<String>, ServerFnError> {
    use super::prelude::*;

    extract_token().await
}
