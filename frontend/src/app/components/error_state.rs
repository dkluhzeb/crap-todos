//! Shared error display with auth detection.

use crate::prelude::*;

fn is_auth_error(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("access denied")
        || lower.contains("permission")
        || lower.contains("authentication")
        || lower.contains("expired token")
        || lower.contains("not authenticated")
        || lower.contains("invalid token")
}

/// Renders an auth-aware error state. Shows a login prompt for auth errors,
/// a generic message otherwise.
#[component]
pub fn ErrorState(
    err: ServerFnError,
    #[prop(default = "Something went wrong")]
    fallback_label: &'static str,
) -> impl IntoView {
    let msg = format!("{err}");

    if is_auth_error(&msg) {
        view! {
            <div class="flex flex-col items-center justify-center py-20 gap-4">
                <div class="w-16 h-16 rounded-full bg-gray-800 flex items-center justify-center">
                    <LogIn class="w-8 h-8 text-gray-500" />
                </div>
                <p class="text-gray-400">"You need to log in first"</p>
                <a href="/login" class="px-4 py-2 bg-amber-600 hover:bg-amber-500 text-white text-sm font-medium rounded-lg transition-colors">
                    "Log in"
                </a>
            </div>
        }.into_any()
    } else {
        view! {
            <div class="flex flex-col items-center justify-center py-20 gap-4">
                <p class="text-red-400 text-sm">{format!("{fallback_label}: {err}")}</p>
            </div>
        }.into_any()
    }
}
