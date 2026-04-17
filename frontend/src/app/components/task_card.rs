//! Kanban task card component.

use crate::prelude::*;
use super::shared::{COLUMNS, column_index, priority_color};

#[component]
pub fn TaskCard(
    task: Tasks,
    on_select: WriteSignal<Option<Tasks>>,
    on_status_change: Callback<(String, String)>,
) -> impl IntoView {
    let overdue = task.overdue.unwrap_or(false);
    let border = if overdue { "border-l-red-500" } else { "border-l-transparent" };
    let task_clone = task.clone();
    let idx = column_index(&task.status);
    let prio = task.priority as i64;
    let prio_color = priority_color(task.priority);
    let cat_name = rel_category_name(&task.category);
    let cat_color = rel_category_color(&task.category).unwrap_or_else(|| "#6b7280".to_string());
    let assignee = rel_user_name(&task.assignee);

    view! {
        <div
            class=format!(
                "bg-gray-800/50 rounded-lg p-3 border border-gray-700/40 border-l-2 {border} hover:bg-gray-800/80 hover:border-gray-600/60 transition-all cursor-pointer group",
            )
            on:click=move |_| on_select.set(Some(task_clone.clone()))
        >

            <div class="flex items-start justify-between gap-2 mb-1.5">
                <h3 class="text-sm font-medium text-gray-200 leading-snug">{task.title.clone()}</h3>
                <div
                    class=format!("flex items-center gap-px flex-shrink-0 {prio_color}")
                    title=format!("Priority {prio}")
                >
                    {(0..prio.min(5)).map(|_| view! { <Flame class="w-3 h-3" /> }).collect_view()}
                </div>
            </div>

            <CardMeta
                cat_name=cat_name
                cat_color=cat_color
                due_date=task.due_date.clone()
                overdue=overdue
                assignee=assignee
            />

            <div class="flex items-center gap-1 mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <MoveButton
                    task_id=task.id.clone()
                    idx=idx
                    direction=-1
                    on_status_change=on_status_change
                />
                <MoveButton
                    task_id=task.id.clone()
                    idx=idx
                    direction=1
                    on_status_change=on_status_change
                />
            </div>
        </div>
    }
}

/// Category badge, due date, and assignee row.
#[component]
fn CardMeta(
    cat_name: Option<String>,
    cat_color: String,
    due_date: Option<String>,
    overdue: bool,
    assignee: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 flex-wrap text-xs">
            {cat_name
                .map(|name| {
                    let c = cat_color.clone();
                    view! {
                        <span
                            class="flex items-center gap-1 px-1.5 py-0.5 rounded"
                            style=format!("background-color: {c}15; color: {c}")
                        >
                            <Tag class="w-2.5 h-2.5" />
                            {name}
                        </span>
                    }
                })}
            {due_date
                .map(|date| {
                    let color = if overdue { "text-red-400" } else { "text-gray-500" };
                    view! {
                        <span class=format!("flex items-center gap-1 {color}")>
                            <Calendar class="w-2.5 h-2.5" />
                            {format_date(&date)}
                        </span>
                    }
                })}
            {assignee
                .map(|name| {
                    view! {
                        <span class="flex items-center gap-1 text-gray-500 ml-auto">
                            <User class="w-2.5 h-2.5" />
                            {name}
                        </span>
                    }
                })}
        </div>
    }
}

/// Move left/right button. Renders nothing if the move isn't possible.
#[component]
fn MoveButton(
    task_id: String,
    idx: usize,
    /// -1 for left, +1 for right
    direction: i32,
    on_status_change: Callback<(String, String)>,
) -> impl IntoView {
    let target_idx = idx as i32 + direction;

    if target_idx < 0 || target_idx >= COLUMNS.len() as i32 {
        return None;
    }

    let target_status = COLUMNS[target_idx as usize].0.to_string();
    let (icon_view, title) = if direction < 0 {
        (
            view! { <ChevronLeft class="w-3 h-3" /> }.into_any(),
            "Move left",
        )
    } else {
        (
            view! { <ChevronRight class="w-3 h-3" /> }.into_any(),
            "Move right",
        )
    };

    Some(view! {
        <button
            class="p-1 rounded bg-gray-700/40 hover:bg-gray-700 text-gray-400 transition-colors"
            title=title
            on:click=move |ev| {
                ev.stop_propagation();
                on_status_change.run((task_id.clone(), target_status.clone()));
            }
        >
            {icon_view}
        </button>
    })
}
