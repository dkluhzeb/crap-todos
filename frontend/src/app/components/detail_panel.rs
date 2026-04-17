//! Task detail slide-out panel with editing controls.

use crate::api::{DeleteTask, UpdateTask, UpdateTaskRelation, fetch_categories, fetch_users};
use crate::prelude::*;
use super::{comments::CommentsSection, shared::{COLUMNS, priority_color}};

#[component]
pub fn DetailPanel(
    task: Tasks,
    on_close: impl Fn() + 'static + Clone,
    update_action: ServerAction<UpdateTask>,
    delete_action: ServerAction<DeleteTask>,
) -> impl IntoView {
    let active_color = priority_color(task.priority);
    let id = task.id.clone();
    let on_close_overlay = on_close.clone();
    let on_close_btn = on_close.clone();

    view! {
        <div
            class="fixed inset-0 bg-black/60 backdrop-blur-sm z-40"
            on:click=move |_| on_close_overlay()
        />
        <div class="fixed right-0 top-0 h-full w-full max-w-lg bg-gray-900 border-l border-gray-800 z-50 overflow-y-auto shadow-2xl">
            <div class="p-6">
                <TitleEditor
                    task_id=id.clone()
                    initial=task.title.clone()
                    update_action=update_action
                    on_close=on_close_btn
                />

                <div class="grid grid-cols-2 gap-4 mb-6">
                    <div>
                        <FieldLabel text="Status" />
                        <StatusSelect
                            current=task.status.clone()
                            task_id=id.clone()
                            update_action=update_action
                        />
                    </div>
                    <div>
                        <FieldLabel text="Priority" />
                        <PrioritySelect
                            current=task.priority as i64
                            task_id=id.clone()
                            update_action=update_action
                            active_color=active_color
                        />
                    </div>
                </div>

                <div class="space-y-4 mb-6">
                    <AssigneePicker
                        task_id=id.clone()
                        current=rel_id(&task.assignee).unwrap_or_default()
                    />
                    <CategoryPicker
                        task_id=id.clone()
                        current=rel_id(&task.category).unwrap_or_default()
                    />
                    <DueDatePicker
                        task_id=id.clone()
                        current=task.due_date.clone().unwrap_or_default()
                        update_action=update_action
                    />
                </div>

                <CommentsSection task_id=task.id.clone() />

                <TrashButton task_id=task.id.clone() delete_action=delete_action />
            </div>
        </div>
    }
}

#[component]
fn TitleEditor(
    task_id: String,
    initial: String,
    update_action: ServerAction<UpdateTask>,
    on_close: impl Fn() + 'static,
) -> impl IntoView {
    let (editing, set_editing) = signal(false);
    let (value, set_value) = signal(initial);

    let on_keydown = {
        let id = task_id;
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Enter" {
                update_action.dispatch(UpdateTask {
                    id: id.clone(),
                    title: Some(value.get_untracked()),
                    status: None,
                    priority: None,
                    due_date: None,
                });
                set_editing.set(false);
            } else if ev.key() == "Escape" {
                set_editing.set(false);
            }
        }
    };

    view! {
        <div class="flex items-start justify-between mb-6">
            <div class="flex-1">
                {move || {
                    if editing.get() {
                        view! {
                            <input
                                type="text"
                                class="w-full bg-gray-800 text-lg font-semibold text-gray-100 px-3 py-2 rounded-lg border border-gray-600 outline-none focus:border-amber-500 focus:ring-1 focus:ring-amber-500/20"
                                prop:value=move || value.get()
                                on:input=move |ev| set_value.set(event_target_value(&ev))
                                on:keydown=on_keydown.clone()
                                autofocus
                            />
                        }
                            .into_any()
                    } else {
                        view! {
                            <h2
                                class="flex items-center gap-2 text-lg font-semibold text-gray-100 cursor-pointer hover:text-amber-400 transition-colors group"
                                on:click=move |_| set_editing.set(true)
                                title="Click to edit"
                            >
                                {value.get()}
                                <Pencil class="w-3.5 h-3.5 text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity" />
                            </h2>
                        }
                            .into_any()
                    }
                }}
            </div>
            <button
                class="ml-4 p-1.5 rounded-lg text-gray-500 hover:text-gray-300 hover:bg-gray-800 transition-colors"
                on:click=move |_| on_close()
            >
                <X class="w-5 h-5" />
            </button>
        </div>
    }
}

#[component]
fn AssigneePicker(task_id: String, current: String) -> impl IntoView {
    let users = Resource::new(|| (), |_| fetch_users());
    let action = ServerAction::<UpdateTaskRelation>::new();

    view! {
        <div>
            <label class="flex items-center gap-1.5 text-xs font-medium text-gray-500 mb-1.5 uppercase tracking-wider">
                <User class="w-3 h-3" />
                "Assignee"
            </label>
            <Suspense fallback=|| {
                view! { <div class="h-9 bg-gray-800 rounded-lg animate-pulse"></div> }
            }>
                {move || {
                    let cur = current.clone();
                    let id = task_id.clone();
                    users
                        .get()
                        .map(|result| match result {
                            Ok(list) => {
                                view! {
                                    <select
                                        class="w-full bg-gray-800 text-sm text-gray-300 px-3 py-2 rounded-lg border border-gray-700 outline-none focus:border-amber-500 appearance-none cursor-pointer"
                                        on:change=move |ev| {
                                            action
                                                .dispatch(UpdateTaskRelation {
                                                    id: id.clone(),
                                                    assignee: Some(event_target_value(&ev)),
                                                    category: None,
                                                });
                                        }
                                    >
                                        <option value="" selected=cur.is_empty()>
                                            "Unassigned"
                                        </option>
                                        {list
                                            .iter()
                                            .map(|u| {
                                                let sel = u.id == cur;
                                                view! {
                                                    <option value=u.id.clone() selected=sel>
                                                        {u.name.clone()}
                                                    </option>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                }
                                    .into_any()
                            }
                            Err(_) => {
                                view! {
                                    <span class="text-xs text-gray-600">"Failed to load"</span>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn CategoryPicker(task_id: String, current: String) -> impl IntoView {
    let categories = Resource::new(|| (), |_| fetch_categories());
    let action = ServerAction::<UpdateTaskRelation>::new();

    view! {
        <div>
            <label class="flex items-center gap-1.5 text-xs font-medium text-gray-500 mb-1.5 uppercase tracking-wider">
                <Tag class="w-3 h-3" />
                "Category"
            </label>
            <Suspense fallback=|| {
                view! { <div class="h-9 bg-gray-800 rounded-lg animate-pulse"></div> }
            }>
                {move || {
                    let cur = current.clone();
                    let id = task_id.clone();
                    categories
                        .get()
                        .map(|result| match result {
                            Ok(list) => {
                                view! {
                                    <select
                                        class="w-full bg-gray-800 text-sm text-gray-300 px-3 py-2 rounded-lg border border-gray-700 outline-none focus:border-amber-500 appearance-none cursor-pointer"
                                        on:change=move |ev| {
                                            action
                                                .dispatch(UpdateTaskRelation {
                                                    id: id.clone(),
                                                    assignee: None,
                                                    category: Some(event_target_value(&ev)),
                                                });
                                        }
                                    >
                                        <option value="" selected=cur.is_empty()>
                                            "No category"
                                        </option>
                                        {list
                                            .iter()
                                            .map(|c| {
                                                let sel = c.id == cur;
                                                view! {
                                                    <option value=c.id.clone() selected=sel>
                                                        {c.name.clone()}
                                                    </option>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                }
                                    .into_any()
                            }
                            Err(_) => {
                                view! {
                                    <span class="text-xs text-gray-600">"Failed to load"</span>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn DueDatePicker(
    task_id: String,
    current: String,
    update_action: ServerAction<UpdateTask>,
) -> impl IntoView {
    view! {
        <div>
            <label class="flex items-center gap-1.5 text-xs font-medium text-gray-500 mb-1.5 uppercase tracking-wider">
                <Calendar class="w-3 h-3" />
                "Due date"
            </label>
            <input
                type="date"
                class="w-full bg-gray-800 text-sm text-gray-300 px-3 py-2 rounded-lg border border-gray-700 outline-none focus:border-amber-500 cursor-pointer"
                value=current
                on:change=move |ev| {
                    update_action
                        .dispatch(UpdateTask {
                            id: task_id.clone(),
                            title: None,
                            status: None,
                            priority: None,
                            due_date: Some(event_target_value(&ev)),
                        });
                }
            />
        </div>
    }
}

#[component]
fn StatusSelect(
    current: String,
    task_id: String,
    update_action: ServerAction<UpdateTask>,
) -> impl IntoView {
    view! {
        <select
            class="w-full bg-gray-800 text-sm text-gray-300 px-3 py-2 rounded-lg border border-gray-700 outline-none focus:border-amber-500 appearance-none cursor-pointer"
            on:change=move |ev| {
                update_action
                    .dispatch(UpdateTask {
                        id: task_id.clone(),
                        title: None,
                        status: Some(event_target_value(&ev)),
                        priority: None,
                        due_date: None,
                    });
            }
        >
            {COLUMNS
                .iter()
                .map(|(value, label)| {
                    let sel = *value == current.as_str();
                    view! {
                        <option value=*value selected=sel>
                            {*label}
                        </option>
                    }
                })
                .collect_view()}
        </select>
    }
}

#[component]
fn PrioritySelect(
    current: i64,
    task_id: String,
    update_action: ServerAction<UpdateTask>,
    active_color: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex gap-0.5 py-1">
            {(1..=5)
                .map(|p| {
                    let active = p <= current;
                    let tid = task_id.clone();
                    let color = if active { active_color } else { "text-gray-700" };
                    let hover = if !active { "hover:text-gray-500" } else { "" };
                    view! {
                        <button
                            class=format!(
                                "p-0.5 rounded transition-all hover:scale-110 {color} {hover}",
                            )
                            on:click=move |_| {
                                update_action
                                    .dispatch(UpdateTask {
                                        id: tid.clone(),
                                        title: None,
                                        status: None,
                                        priority: Some(p as f64),
                                        due_date: None,
                                    });
                            }
                            title=format!("Priority {p}")
                        >
                            <Flame class="w-5 h-5" />
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn FieldLabel(text: &'static str) -> impl IntoView {
    view! {
        <label class="block text-xs font-medium text-gray-500 mb-1.5 uppercase tracking-wider">
            {text}
        </label>
    }
}

#[component]
fn TrashButton(task_id: String, delete_action: ServerAction<DeleteTask>) -> impl IntoView {
    view! {
        <div class="pt-4 border-t border-gray-800">
            <button
                class="flex items-center gap-2 text-sm text-gray-600 hover:text-red-400 transition-colors"
                on:click=move |_| {
                    delete_action.dispatch(DeleteTask { id: task_id.clone() });
                }
            >
                <Trash2 class="w-4 h-4" />
                "Move to trash"
            </button>
        </div>
    }
}
