//! Inline SVG icons from Lucide. Each renders a 24x24 SVG that inherits
//! `currentColor` and respects the `class` prop for sizing via Tailwind.

use leptos::prelude::*;

macro_rules! icon {
    ($name:ident, $path:expr) => {
        #[component]
        pub fn $name(#[prop(optional)] class: &'static str) -> impl IntoView {
            let class = if class.is_empty() { "w-4 h-4" } else { class };
            view! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class=class
                >
                    {$path}
                </svg>
            }
            .into_any()
        }
    };
}

icon!(
    LayoutDashboard,
    view! {
        <rect width="7" height="9" x="3" y="3" rx="1" />
        <rect width="7" height="5" x="14" y="3" rx="1" />
        <rect width="7" height="5" x="14" y="12" rx="1" />
        <rect width="7" height="9" x="3" y="16" rx="1" />
    }
);

icon!(
    Trash2,
    view! {
        <path d="M3 6h18" />
        <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
        <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
        <line x1="10" x2="10" y1="11" y2="17" />
        <line x1="14" x2="14" y1="11" y2="17" />
    }
);

icon!(
    Plus,
    view! {
        <path d="M5 12h14" />
        <path d="M12 5v14" />
    }
);

icon!(
    X,
    view! {
        <path d="M18 6 6 18" />
        <path d="m6 6 12 12" />
    }
);

icon!(
    LogOut,
    view! {
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
        <polyline points="16 17 21 12 16 7" />
        <line x1="21" x2="9" y1="12" y2="12" />
    }
);

icon!(
    LogIn,
    view! {
        <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />
        <polyline points="10 17 15 12 10 7" />
        <line x1="15" x2="3" y1="12" y2="12" />
    }
);

// ── Task actions ─────────────────────────────────────────────────────

icon!(
    ChevronLeft,
    view! {
        <path d="m15 18-6-6 6-6" />
    }
);

icon!(
    ChevronRight,
    view! {
        <path d="m9 18 6-6-6-6" />
    }
);

icon!(
    Pencil,
    view! {
        <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" />
        <path d="m15 5 4 4" />
    }
);

icon!(
    Calendar,
    view! {
        <path d="M8 2v4" />
        <path d="M16 2v4" />
        <rect width="18" height="18" x="3" y="4" rx="2" />
        <path d="M3 10h18" />
    }
);

icon!(
    User,
    view! {
        <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
        <circle cx="12" cy="7" r="4" />
    }
);

icon!(
    Tag,
    view! {
        <path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z" />
        <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
    }
);

icon!(
    ArchiveRestore,
    view! {
        <rect width="20" height="5" x="2" y="3" rx="1" />
        <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" />
        <path d="m9.5 17 2.5-2.5L14.5 17" />
        <path d="M12 14.5V11" />
    }
);

icon!(
    Flame,
    view! {
        <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" />
    }
);

icon!(
    CircleDot,
    view! {
        <circle cx="12" cy="12" r="10" />
        <circle cx="12" cy="12" r="1" />
    }
);

icon!(
    Construction,
    view! {
        <rect x="2" y="6" width="20" height="8" rx="1" />
        <path d="M17 14v7" />
        <path d="M7 14v7" />
        <path d="M17 3v3" />
        <path d="M7 3v3" />
        <path d="M10 14 2.3 6.3" />
        <path d="m14 6 7.7 7.7" />
        <path d="m8 6 8 8" />
    }
);
