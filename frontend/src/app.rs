//! Main application component and routing.

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub mod components;
pub mod icons;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Crap Todo" />
        <Router>
            <main class="min-h-screen">
                <Routes fallback=|| {
                    view! {
                        <div class="flex flex-col items-center justify-center min-h-screen gap-4">
                            <div class="w-16 h-16 rounded-full bg-gray-800 flex items-center justify-center">
                                <icons::X class="w-8 h-8 text-gray-500" />
                            </div>
                            <h1 class="text-xl font-bold text-gray-300">"404"</h1>
                            <p class="text-gray-500 text-sm">
                                "Task not found (just like your motivation)"
                            </p>
                            <a
                                href="/"
                                class="flex items-center gap-1.5 text-sm text-amber-500 hover:text-amber-400 transition-colors"
                            >
                                <icons::LayoutDashboard class="w-3.5 h-3.5" />
                                "Back to board"
                            </a>
                        </div>
                    }
                }>
                    <Route path=path!("/") view=pages::Board />
                    <Route path=path!("/login") view=pages::Login />
                    <Route path=path!("/trash") view=pages::Trash />
                </Routes>
            </main>
        </Router>
    }
}
