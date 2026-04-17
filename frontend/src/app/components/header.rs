//! Shared app header with navigation, live indicator, and user menu.

use crate::api::Logout;
use crate::prelude::*;
use super::live::LiveUpdates;

fn nav_class(is_active: bool) -> &'static str {
    if is_active {
        "text-gray-300 hover:text-white"
    } else {
        "text-gray-500 hover:text-gray-300"
    }
}

#[component]
pub fn Header(
    user: Resource<Result<Option<Users>, ServerFnError>>,
    logout_action: ServerAction<Logout>,
    on_live_event: Callback<()>,
    #[prop(default = "board")] active: &'static str,
) -> impl IntoView {
    view! {
        <header class="flex items-center justify-between px-6 py-3 border-b border-gray-800 bg-gray-900/80 backdrop-blur-sm">
            <Brand on_live_event=on_live_event />
            <Nav active=active user=user logout_action=logout_action />
        </header>
    }
}

#[component]
fn Brand(on_live_event: Callback<()>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            <LayoutDashboard class="w-5 h-5 text-amber-500" />
            <h1 class="text-lg font-bold tracking-tight">"Crap Todo"</h1>
            <LiveUpdates on_event=on_live_event />
        </div>
    }
}

#[component]
fn Nav(
    active: &'static str,
    user: Resource<Result<Option<Users>, ServerFnError>>,
    logout_action: ServerAction<Logout>,
) -> impl IntoView {
    view! {
        <nav class="flex items-center gap-5">
            <NavLink href="/" label="Board" active=active == "board">
                <LayoutDashboard class="w-3.5 h-3.5" />
            </NavLink>
            <NavLink href="/trash" label="Trash" active=active == "trash">
                <Trash2 class="w-3.5 h-3.5" />
            </NavLink>
            <UserMenu user=user logout_action=logout_action />
        </nav>
    }
}

#[component]
fn NavLink(
    href: &'static str,
    label: &'static str,
    active: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            href=href
            class=format!(
                "flex items-center gap-1.5 text-sm transition-colors {}",
                nav_class(active),
            )
        >
            {children()}
            {label}
        </a>
    }
}

#[component]
fn UserMenu(
    user: Resource<Result<Option<Users>, ServerFnError>>,
    logout_action: ServerAction<Logout>,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| ()>
            {move || {
                user
                    .get()
                    .map(|result| match result {
                        Ok(Some(u)) => {
                            view! {
                                <div class="flex items-center gap-3 pl-4 border-l border-gray-700">
                                    <div class="flex items-center gap-1.5">
                                        <User class="w-3.5 h-3.5 text-gray-500" />
                                        <span class="text-sm text-gray-400">{u.name.clone()}</span>
                                    </div>
                                    <ActionForm action=logout_action>
                                        <button
                                            type="submit"
                                            class="flex items-center gap-1 text-xs text-gray-600 hover:text-red-400 transition-colors"
                                        >
                                            <LogOut class="w-3 h-3" />
                                            "Logout"
                                        </button>
                                    </ActionForm>
                                </div>
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <a
                                    href="/login"
                                    class="flex items-center gap-1.5 text-sm text-amber-500 hover:text-amber-400 transition-colors"
                                >
                                    <LogIn class="w-3.5 h-3.5" />
                                    "Log in"
                                </a>
                            }
                                .into_any()
                        }
                    })
            }}
        </Suspense>
    }
}
