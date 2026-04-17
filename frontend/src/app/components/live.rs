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
        {
            // Close any previous EventSource before creating a new one.
            // This prevents connection leaks on page navigation.
            close_existing_sse();
            if let Some(token) = token_opt {
                connect_sse(&token, on_event, set_connected);
            }
        }
    });

    #[cfg(feature = "hydrate")]
    on_cleanup(close_existing_sse);

    view! {
        <div class="flex items-center gap-1.5">
            <div class=move || {
                if connected.get() { "w-1.5 h-1.5 rounded-full bg-green-500" }
                else { "w-1.5 h-1.5 rounded-full bg-gray-600" }
            } />
            <span class="text-xs text-gray-600">
                {move || if connected.get() { "live" } else { "" }}
            </span>
        </div>
    }
}

#[cfg(feature = "hydrate")]
thread_local! {
    static CURRENT_ES: std::cell::RefCell<Option<web_sys::EventSource>> = const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "hydrate")]
fn close_existing_sse() {
    CURRENT_ES.with(|cell| {
        if let Some(es) = cell.borrow_mut().take() {
            es.close();
        }
    });
}

#[cfg(feature = "hydrate")]
fn connect_sse(token: &str, on_event: Callback<()>, set_connected: WriteSignal<bool>) {
    use wasm_bindgen::prelude::*;
    use web_sys::EventSource;

    let url = format!("/events?token={token}");

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
    es.add_event_listener_with_callback("mutation", on_mutation.as_ref().unchecked_ref()).ok();
    on_mutation.forget();

    let on_error = Closure::wrap(Box::new(move |_: web_sys::Event| {
        set_connected.set(false);
    }) as Box<dyn FnMut(_)>);
    es.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();

    CURRENT_ES.with(|cell| {
        *cell.borrow_mut() = Some(es);
    });
}
