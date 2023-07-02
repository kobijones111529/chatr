use leptos::*;
use crate::ws::{send_msg, use_ws};

/// Renders the home page of your application.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    use_ws(cx);

    view! { cx,
        <button on:click=move |_| {
            send_msg(cx, "Hi!");
        }>"Send"</button>
    }
}
