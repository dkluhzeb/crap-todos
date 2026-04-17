//! SSR entrypoint -- Axum server that renders Leptos and proxies SSE events.

#[cfg(feature = "ssr")]
use axum::{
    Router,
    http::{HeaderMap, header::COOKIE},
    routing, serve,
};
#[cfg(feature = "ssr")]
use crap_todos_frontend::{app::App, sse, types::AuthToken};
#[cfg(feature = "ssr")]
use futures::FutureExt as _;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{LeptosRoutes, extract, generate_route_list, handle_server_fns};
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
#[cfg(feature = "ssr")]
use tokio::net::TcpListener;
#[cfg(feature = "ssr")]
use tower_http::services::ServeDir;
#[cfg(feature = "ssr")]
use tracing::info;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    init_tracing();

    let conf = get_configuration(Some("Cargo.toml")).expect("Can not terminate config dir");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/{*fn_name}", routing::post(handle_server_fns))
        .route("/events", routing::get(sse::events_handler))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            provide_auth_context,
            shell(leptos_options.clone()),
        )
        .fallback(routing::get_service(ServeDir::new(
            &*leptos_options.site_root,
        )))
        .with_state(leptos_options);

    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on http://{}", &addr);

    serve(listener, app.into_make_service()).await.unwrap();
}

/// Extract the auth token from the request cookie and provide it as Leptos
/// context so server functions can access it during SSR.
#[cfg(feature = "ssr")]
fn provide_auth_context() {
    let token = extract::<HeaderMap>()
        .now_or_never()
        .and_then(|r| r.ok())
        .and_then(|headers| {
            headers
                .get_all(COOKIE)
                .iter()
                .filter_map(|v| v.to_str().ok())
                .flat_map(|s| s.split(';'))
                .map(|s| s.trim())
                .find(|s| s.starts_with("crap_token="))
                .map(|s| s.trim_start_matches("crap_token=").to_string())
        });

    provide_context(AuthToken(token));
}

/// Build the HTML shell rendered around the Leptos app.
#[cfg(feature = "ssr")]
fn shell(options: LeptosOptions) -> impl Fn() -> AnyView + Clone + Send + 'static {
    move || {
        view! {
            <!DOCTYPE html>
            <html lang="en" class="dark">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options=options.clone() />
                    <MetaTags />
                    <link rel="stylesheet" id="leptos" href="/pkg/crap-todos.css" />
                </head>
                <body class="bg-gray-950 text-gray-100 min-h-screen">
                    <App />
                </body>
            </html>
        }
        .into_any()
    }
}

#[cfg(feature = "ssr")]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
