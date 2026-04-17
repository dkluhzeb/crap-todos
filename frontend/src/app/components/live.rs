//! Live update indicator -- connects to SSE and triggers task refetches.

use crate::api::get_sse_token;
use crate::prelude::*;

#[component]
pub fn LiveUpdates(on_event: Callback<()>) -> impl IntoView {
    let (connected, set_connected) = signal(false);
    let token = Resource::new(|| (), |_| get_sse_token());

    Effect::new(move || {
        let Some(Ok(token_opt)) = token.get() else {
            return;
        };

        // SSR compiles out the hydrate block; suppress unused warnings.
        let _ = (&on_event, &set_connected, &token_opt);

        #[cfg(feature = "hydrate")]
        connect_sse(token_opt, on_event, set_connected);
    });

    view! {
        <div class="flex items-center gap-1.5">
            <div class=move || {
                if connected.get() {
                    "w-1.5 h-1.5 rounded-full bg-green-500"
                } else {
                    "w-1.5 h-1.5 rounded-full bg-gray-600"
                }
            } />
            <span class="text-xs text-gray-600">
                {move || if connected.get() { "live" } else { "" }}
            </span>
        </div>
    }
}

/// Wire up an EventSource to the SSE endpoint.
#[cfg(feature = "hydrate")]
fn connect_sse(token: Option<String>, on_event: Callback<()>, set_connected: WriteSignal<bool>) {
    use wasm_bindgen::prelude::*;
    use web_sys::EventSource;

    let url = match token {
        Some(t) => format!("/events?token={t}"),
        None => "/events".to_string(),
    };

    let es = EventSource::new(&url).unwrap_or_else(|e| {
        leptos::logging::warn!("EventSource failed: {:?}", e);
        panic!("EventSource creation failed");
    });

    let on_open = Closure::wrap(Box::new(move |_: web_sys::Event| {
        set_connected.set(true);
    }) as Box<dyn FnMut(_)>);

    es.set_onopen(Some(on_open.as_ref().unchecked_ref()));

    on_open.forget();

    let on_mutation = Closure::wrap(Box::new(move |_: web_sys::MessageEvent| {
        on_event.run(());
    }) as Box<dyn FnMut(_)>);

    es.add_event_listener_with_callback("mutation", on_mutation.as_ref().unchecked_ref())
        .ok();

    on_mutation.forget();

    let on_error = Closure::wrap(Box::new(move |_: web_sys::Event| {
        set_connected.set(false);
    }) as Box<dyn FnMut(_)>);

    es.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    on_error.forget();
}
