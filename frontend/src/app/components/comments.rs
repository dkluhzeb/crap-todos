//! Comments section for task detail panel.

use crate::api::{CreateComment, fetch_comments};
use crate::prelude::*;

#[component]
pub fn CommentsSection(task_id: String) -> impl IntoView {
    let tid = task_id.clone();
    let comments = Resource::new(move || tid.clone(), fetch_comments);
    let create_action = ServerAction::<CreateComment>::new();
    let (new_comment, set_new_comment) = signal(String::new());
    let task_id_for_submit = task_id.clone();

    Effect::new(move || {
        if let Some(Ok(())) = create_action.value().get() {
            comments.refetch();
            set_new_comment.set(String::new());
        }
    });

    let submit = move |body: String| {
        if !body.is_empty() {
            create_action.dispatch(CreateComment { task_id: task_id_for_submit.clone(), body });
        }
    };
    let submit_key = submit.clone();
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" { submit_key(new_comment.get_untracked()); }
    };
    let on_send = move |_| { submit(new_comment.get_untracked()); };

    view! {
        <div class="pt-4 mb-4 border-t border-gray-800">
            <h3 class="text-xs font-medium text-gray-500 mb-3 uppercase tracking-wider">
                "Comments"
            </h3>

            <CommentList comments=comments />

            <CommentInput
                new_comment=new_comment
                set_new_comment=set_new_comment
                on_keydown=on_keydown
                on_send=on_send
                pending=create_action.pending()
            />
        </div>
    }
}

#[component]
fn CommentList(comments: Resource<Result<Vec<Comments>, ServerFnError>>) -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! { <div class="h-8 bg-gray-800/40 rounded animate-pulse mb-3"></div> }
        }>
            {move || {
                comments
                    .get()
                    .map(|result| match result {
                        Ok(list) if list.is_empty() => {
                            view! { <p class="text-xs text-gray-600 mb-3">"No comments yet"</p> }
                                .into_any()
                        }
                        Ok(list) => {
                            list.into_iter()
                                .map(|c| view! { <CommentBubble comment=c /> })
                                .collect_view()
                                .into_any()
                        }
                        Err(_) => {
                            view! {
                                <p class="text-xs text-red-400 mb-3">"Failed to load comments"</p>
                            }
                                .into_any()
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
fn CommentBubble(comment: Comments) -> impl IntoView {
    let author = comment.author.as_doc().map(|u| u.name.clone()).unwrap_or_else(|| "Unknown".to_string());
    let time = comment.created_at.as_deref().map(format_date).unwrap_or_default();

    view! {
        <div class="mb-3 p-3 bg-gray-800/40 rounded-lg">
            <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium text-gray-400 flex items-center gap-1">
                    <User class="w-3 h-3" />
                    {author}
                </span>
                <span class="text-xs text-gray-600">{time}</span>
            </div>
            <p class="text-sm text-gray-300">{comment.body}</p>
        </div>
    }
}

#[component]
fn CommentInput(
    new_comment: ReadSignal<String>,
    set_new_comment: WriteSignal<String>,
    on_keydown: impl Fn(leptos::ev::KeyboardEvent) + 'static,
    on_send: impl Fn(leptos::ev::MouseEvent) + 'static,
    pending: Memo<bool>,
) -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <input
                type="text"
                class="flex-1 bg-gray-800 text-sm text-gray-300 px-3 py-2 rounded-lg border border-gray-700 outline-none focus:border-amber-500 placeholder-gray-600"
                placeholder="Add a comment..."
                prop:value=move || new_comment.get()
                on:input=move |ev| set_new_comment.set(event_target_value(&ev))
                on:keydown=on_keydown
            />
            <button
                class="px-3 py-2 bg-gray-800 hover:bg-gray-700 text-gray-400 rounded-lg text-sm transition-colors disabled:opacity-50"
                disabled=move || new_comment.get().is_empty() || pending.get()
                on:click=on_send
            >
                "Send"
            </button>
        </div>
    }
}
