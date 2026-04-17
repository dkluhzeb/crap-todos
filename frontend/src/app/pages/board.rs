//! Kanban board page.

use leptos_router::hooks::use_navigate;
use crate::api::{
    CreateTask, DeleteTask, Logout, UpdateTask, fetch_tasks, get_current_user, update_task_status,
};
use crate::app::components::{DetailPanel, ErrorState, Header, TaskCard, shared::COLUMNS};
use crate::prelude::*;

#[component]
pub fn Board() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_current_user());
    let tasks = Resource::new(|| (), |_| fetch_tasks());
    let create_action = ServerAction::<CreateTask>::new();
    let delete_action = ServerAction::<DeleteTask>::new();
    let update_action = ServerAction::<UpdateTask>::new();
    let logout_action = ServerAction::<Logout>::new();
    let (selected_task, set_selected_task) = signal::<Option<Tasks>>(None);

    Effect::new(move || {
        if let Some(Ok(_)) = create_action.value().get() {
            tasks.refetch();
        }
    });

    Effect::new(move || {
        if let Some(Ok(_)) = delete_action.value().get() {
            tasks.refetch();
            set_selected_task.set(None);
        }
    });

    Effect::new(move || {
        if let Some(Ok(updated)) = update_action.value().get() {
            tasks.refetch();
            set_selected_task.set(Some(updated));
        }
    });

    let navigate = use_navigate();
    Effect::new(move || {
        if let Some(Ok(())) = logout_action.value().get() {
            navigate("/login", Default::default());
        }
    });

    let on_live_event = Callback::new(move |()| tasks.refetch());

    let on_status_change = Callback::new(move |(id, status): (String, String)| {
        leptos::task::spawn_local(async move {
            let _ = update_task_status(id, status).await;
            tasks.refetch();
        });
    });

    view! {
        <div class="flex flex-col h-screen">
            <Header
                user=user
                logout_action=logout_action
                on_live_event=on_live_event
                active="board"
            />

            <div class="flex-1 overflow-x-auto p-6">
                <TaskBoard
                    tasks=tasks
                    create_action=create_action
                    on_select=set_selected_task
                    on_status_change=on_status_change
                />
            </div>

            {move || {
                selected_task
                    .get()
                    .map(|task| {
                        view! {
                            <DetailPanel
                                task=task
                                on_close=move || set_selected_task.set(None)
                                update_action=update_action
                                delete_action=delete_action
                            />
                        }
                    })
            }}
        </div>
    }
}

#[component]
fn TaskBoard(
    tasks: Resource<Result<Vec<Tasks>, ServerFnError>>,
    create_action: ServerAction<CreateTask>,
    on_select: WriteSignal<Option<Tasks>>,
    on_status_change: Callback<(String, String)>,
) -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! { <BoardSkeleton /> }
        }>
            {move || {
                tasks
                    .get()
                    .map(|result| match result {
                        Ok(all_tasks) => {
                            view! {
                                <div class="flex gap-4 h-full min-w-max">
                                    {COLUMNS
                                        .iter()
                                        .map(|(status, label)| {
                                            let col_tasks: Vec<Tasks> = all_tasks
                                                .iter()
                                                .filter(|t| t.status == *status)
                                                .cloned()
                                                .collect();
                                            view! {
                                                <Column
                                                    label=label.to_string()
                                                    tasks=col_tasks
                                                    create_action=create_action
                                                    status=status.to_string()
                                                    on_select=on_select
                                                    on_status_change=on_status_change
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <ErrorState err=e fallback_label="Failed to load" /> }.into_any(),
                    })
            }}
        </Suspense>
    }
}

#[component]
fn BoardSkeleton() -> impl IntoView {
    view! {
        <div class="flex gap-4 h-full min-w-max">
            {COLUMNS
                .iter()
                .map(|_| {
                    view! {
                        <div class="w-80 flex-shrink-0 flex flex-col bg-gray-900/30 rounded-xl border border-gray-800 animate-pulse">
                            <div class="px-4 py-3 border-b border-gray-800">
                                <div class="h-4 bg-gray-800 rounded w-24"></div>
                            </div>
                            <div class="p-3 space-y-3">
                                <div class="h-16 bg-gray-800/40 rounded-lg"></div>
                                <div class="h-16 bg-gray-800/40 rounded-lg"></div>
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Column(
    label: String,
    tasks: Vec<Tasks>,
    create_action: ServerAction<CreateTask>,
    status: String,
    on_select: WriteSignal<Option<Tasks>>,
    on_status_change: Callback<(String, String)>,
) -> impl IntoView {
    let count = tasks.len();
    let (show_input, set_show_input) = signal(false);

    view! {
        <div class="w-80 flex-shrink-0 flex flex-col bg-gray-900/30 rounded-xl border border-gray-800 max-h-[calc(100vh-7rem)]">
            <ColumnHeader label=label count=count on_add=move || set_show_input.set(true) />

            <div class="flex-1 p-2 space-y-2 overflow-y-auto">
                {move || {
                    show_input
                        .get()
                        .then(|| {
                            view! {
                                <CreateInput
                                    status=status.clone()
                                    create_action=create_action
                                    on_dismiss=move || set_show_input.set(false)
                                />
                            }
                        })
                }}
                {if tasks.is_empty() && !show_input.get_untracked() {
                    view! {
                        <p class="text-xs text-gray-600 text-center py-8">"Nothing here yet"</p>
                    }
                        .into_any()
                } else {
                    tasks
                        .into_iter()
                        .map(|task| {
                            view! {
                                <TaskCard
                                    task=task
                                    on_select=on_select
                                    on_status_change=on_status_change
                                />
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
        </div>
    }
}

#[component]
fn ColumnHeader(label: String, count: usize, on_add: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between px-4 py-3 border-b border-gray-800">
            <div class="flex items-center gap-2">
                <h2 class="font-semibold text-sm text-gray-300">{label}</h2>
                <span class="text-xs text-gray-600 bg-gray-800/80 px-2 py-0.5 rounded-full">
                    {count}
                </span>
            </div>
            <button
                class="p-1 rounded-md text-gray-600 hover:text-gray-300 hover:bg-gray-800 transition-colors"
                on:click=move |_| on_add()
                title="Add task"
            >
                <Plus class="w-4 h-4" />
            </button>
        </div>
    }
}

#[component]
fn CreateInput(
    status: String,
    create_action: ServerAction<CreateTask>,
    on_dismiss: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let (new_title, set_new_title) = signal(String::new());
    let on_dismiss_esc = on_dismiss.clone();

    let on_keydown = {
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Enter" {
                let title = new_title.get_untracked();

                if !title.is_empty() {
                    create_action.dispatch(CreateTask {
                        title,
                        status: status.clone(),
                    });

                    set_new_title.set(String::new());

                    on_dismiss();
                }
            } else if ev.key() == "Escape" {
                on_dismiss_esc();
            }
        }
    };

    view! {
        <div class="bg-gray-800/80 rounded-lg p-3 border border-gray-600 ring-1 ring-amber-500/20">
            <input
                type="text"
                class="w-full bg-transparent text-sm text-gray-200 placeholder-gray-500 outline-none"
                placeholder="What needs doing?"
                autofocus
                prop:value=move || new_title.get()
                on:input=move |ev| set_new_title.set(event_target_value(&ev))
                on:keydown=on_keydown
            />
            <p class="text-xs text-gray-600 mt-1.5">"Enter to create * Esc to cancel"</p>
        </div>
    }
}
