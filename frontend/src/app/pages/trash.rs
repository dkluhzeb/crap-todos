//! Trash page -- "Graveyard of Good Intentions"

use leptos_router::hooks::use_navigate;
use crate::api::{HardDeleteTask, Logout, RestoreTask, fetch_trashed_tasks, get_current_user};
use crate::app::components::{ErrorState, Header, shared::{COLUMNS, priority_color, priority_flame_count}};
use crate::prelude::*;

#[component]
pub fn Trash() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_current_user());
    let tasks = Resource::new(|| (), |_| fetch_trashed_tasks());
    let restore_action = ServerAction::<RestoreTask>::new();
    let hard_delete_action = ServerAction::<HardDeleteTask>::new();
    let logout_action = ServerAction::<Logout>::new();

    Effect::new(move || {
        if let Some(Ok(_)) = restore_action.value().get() {
            tasks.refetch();
        }
    });

    Effect::new(move || {
        if let Some(Ok(_)) = hard_delete_action.value().get() {
            tasks.refetch();
        }
    });

    let navigate = use_navigate();
    Effect::new(move || {
        if let Some(Ok(())) = logout_action.value().get() {
            navigate("/login", Default::default());
        }
    });

    let on_live_event = Callback::new(move |()| tasks.refetch());

    view! {
        <div class="flex flex-col h-screen">
            <Header
                user=user
                logout_action=logout_action
                on_live_event=on_live_event
                active="trash"
            />

            <div class="flex-1 p-6 max-w-4xl mx-auto w-full overflow-y-auto">
                <TrashHeader />
                <TrashList
                    tasks=tasks
                    restore_action=restore_action
                    hard_delete_action=hard_delete_action
                />
            </div>
        </div>
    }
}

#[component]
fn TrashHeader() -> impl IntoView {
    view! {
        <div class="mb-6">
            <div class="flex items-center gap-2 mb-1">
                <Trash2 class="w-5 h-5 text-gray-400" />
                <h2 class="text-lg font-semibold text-gray-300">"Graveyard of Good Intentions"</h2>
            </div>
            <p class="text-sm text-gray-600">
                "Tasks that didn't make it. Restore them or let them go forever."
            </p>
        </div>
    }
}

#[component]
fn TrashList(
    tasks: Resource<Result<Vec<Tasks>, ServerFnError>>,
    restore_action: ServerAction<RestoreTask>,
    hard_delete_action: ServerAction<HardDeleteTask>,
) -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! { <TrashSkeleton /> }
        }>
            {move || {
                tasks
                    .get()
                    .map(|result| match result {
                        Ok(list) if list.is_empty() => view! { <TrashEmpty /> }.into_any(),
                        Ok(list) => {
                            view! {
                                <div class="space-y-2">
                                    {list
                                        .into_iter()
                                        .map(|task| {
                                            view! {
                                                <TrashRow
                                                    task=task
                                                    restore_action=restore_action
                                                    hard_delete_action=hard_delete_action
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <ErrorState err=e fallback_label="Failed to load trash" /> }.into_any()
                    })
            }}
        </Suspense>
    }
}

#[component]
fn TrashSkeleton() -> impl IntoView {
    view! {
        <div class="space-y-3">
            <div class="h-16 bg-gray-800/40 rounded-lg animate-pulse"></div>
            <div class="h-16 bg-gray-800/40 rounded-lg animate-pulse"></div>
        </div>
    }
}

#[component]
fn TrashEmpty() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 gap-4">
            <div class="w-16 h-16 rounded-full bg-gray-800 flex items-center justify-center">
                <Trash2 class="w-8 h-8 text-gray-600" />
            </div>
            <p class="text-gray-500">"Trash is empty"</p>
            <p class="text-sm text-gray-600">
                "Nothing to see here. Your tasks are safe on the board."
            </p>
        </div>
    }
}

#[component]
fn TrashRow(
    task: Tasks,
    restore_action: ServerAction<RestoreTask>,
    hard_delete_action: ServerAction<HardDeleteTask>,
) -> impl IntoView {
    let restore_id = task.id.clone();
    let delete_id = task.id.clone();

    view! {
        <div class="flex items-center gap-4 px-4 py-3 bg-gray-900/60 border border-gray-800 rounded-lg group hover:border-gray-700 transition-colors">
            <TrashRowMeta task=task />
            <TrashRowActions
                restore_id=restore_id
                delete_id=delete_id
                restore_action=restore_action
                hard_delete_action=hard_delete_action
            />
        </div>
    }
}

#[component]
fn TrashRowMeta(task: Tasks) -> impl IntoView {
    let label = status_label(&task.status).to_string();
    let cat_name = rel_category_name(&task.category);
    let cat_color = rel_category_color(&task.category);
    let assignee = rel_user_name(&task.assignee);
    let due = task.due_date.clone();
    let flame_count = priority_flame_count(task.priority);

    view! {
        <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
                <span class="text-sm font-medium text-gray-300 truncate">{task.title}</span>
                {(flame_count > 0).then(|| view! {
                    <div class=format!("flex items-center gap-px {}", priority_color(task.priority))>
                        {(0..flame_count).map(|_| view! { <Flame class="w-3 h-3" /> }).collect_view()}
                    </div>
                })}
            </div>

            <div class="flex items-center gap-3 mt-1 text-xs text-gray-600">
                <span>{label}</span>
                {cat_name
                    .map(|name| {
                        let bg = cat_color
                            .map(|c| format!("background-color: {c}20; color: {c}"))
                            .unwrap_or_default();
                        view! {
                            <span class="px-1.5 py-0.5 rounded text-[10px]" style=bg>
                                {name}
                            </span>
                        }
                    })}
                {assignee
                    .map(|name| {
                        view! {
                            <span class="flex items-center gap-1">
                                <User class="w-3 h-3" />
                                {name}
                            </span>
                        }
                    })}
                {due
                    .map(|d| {
                        view! {
                            <span class="flex items-center gap-1">
                                <Calendar class="w-3 h-3" />
                                {format_date(&d)}
                            </span>
                        }
                    })}
            </div>
        </div>
    }
}

#[component]
fn TrashRowActions(
    restore_id: String,
    delete_id: String,
    restore_action: ServerAction<RestoreTask>,
    hard_delete_action: ServerAction<HardDeleteTask>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
                class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-amber-500 bg-amber-500/10 rounded-md hover:bg-amber-500/20 transition-colors"
                on:click=move |_| {
                    restore_action
                        .dispatch(RestoreTask {
                            id: restore_id.clone(),
                        });
                }
            >
                <ArchiveRestore class="w-3.5 h-3.5" />
                "Restore"
            </button>
            <button
                class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-red-400 bg-red-500/10 rounded-md hover:bg-red-500/20 transition-colors"
                on:click=move |_| {
                    hard_delete_action
                        .dispatch(HardDeleteTask {
                            id: delete_id.clone(),
                        });
                }
            >
                <X class="w-3.5 h-3.5" />
                "Delete"
            </button>
        </div>
    }
}

fn status_label(status: &str) -> &str {
    COLUMNS
        .iter()
        .find(|(s, _)| *s == status)
        .map(|(_, label)| *label)
        .unwrap_or(status)
}
