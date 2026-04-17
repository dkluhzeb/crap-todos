//! Login page.

use leptos_router::hooks::use_navigate;
use crate::api;
use crate::prelude::*;

const INPUT_CLASS: &str = "w-full px-3 py-2.5 bg-gray-800 border border-gray-700 rounded-lg text-gray-100 text-sm placeholder-gray-500 outline-none focus:border-amber-500 focus:ring-1 focus:ring-amber-500/20 transition-colors";

#[component]
pub fn Login() -> impl IntoView {
    let action = ServerAction::<api::Login>::new();
    let navigate = use_navigate();

    Effect::new(move || {
        if let Some(Ok(_)) = action.value().get() {
            navigate("/", Default::default());
        }
    });

    let error_msg = move || {
        action
            .value()
            .get()
            .and_then(|r| r.err())
            .map(|e| format!("{e}"))
    };

    let pending = action.pending();

    view! {
        <div class="flex items-center justify-center min-h-screen bg-gray-950">
            <div class="w-full max-w-sm p-8 bg-gray-900 rounded-2xl border border-gray-800 shadow-xl">
                <LoginHeader />

                {move || {
                    error_msg()
                        .map(|msg| {
                            view! {
                                <div class="mb-4 p-3 bg-red-900/20 border border-red-800/50 rounded-lg text-red-300 text-sm">
                                    {msg}
                                </div>
                            }
                        })
                }}

                <ActionForm action=action>
                    <div class="space-y-4">
                        <FormField
                            label="Email"
                            input_type="email"
                            name="email"
                            placeholder="admin@craptodo.local"
                        />
                        <FormField
                            label="Password"
                            input_type="password"
                            name="password"
                            placeholder="crap123"
                        />
                        <SubmitButton pending=pending />
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}

#[component]
fn LoginHeader() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center gap-2 mb-2">
            <LayoutDashboard class="w-6 h-6 text-amber-500" />
            <h1 class="text-2xl font-bold">"Crap Todo"</h1>
        </div>
        <p class="text-gray-500 text-center text-sm mb-8">"Because your tasks deserve better"</p>
    }
}

#[component]
fn FormField(
    label: &'static str,
    input_type: &'static str,
    name: &'static str,
    placeholder: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <label class="block text-xs font-medium text-gray-400 mb-1.5 uppercase tracking-wider">
                {label}
            </label>
            <input type=input_type name=name required class=INPUT_CLASS placeholder=placeholder />
        </div>
    }
}

#[component]
fn SubmitButton(pending: Memo<bool>) -> impl IntoView {
    view! {
        <button
            type="submit"
            class="w-full py-2.5 bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold rounded-lg transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
            disabled=move || pending.get()
        >
            <LogIn class="w-4 h-4" />
            {move || if pending.get() { "Logging in..." } else { "Log in" }}
        </button>
    }
}
