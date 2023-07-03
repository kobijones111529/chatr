use crate::{components::home::*, ws::provide_websocket};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    provide_websocket(cx, "ws://localhost:3002/ws").unwrap();

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Chat-r"/>
        <Meta name="color-scheme" content="dark" />
        <Router>
            // <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home /> }/>
                </Routes>
            // </main>
        </Router>
    }
}
