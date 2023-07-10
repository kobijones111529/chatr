use crate::components::home::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Connect to WebSocket if on client
    #[cfg(not(feature = "ssr"))]
    (move || {
        use crate::ws::provide_websocket;

        let location = window().location();
        let protocol = location.protocol().map(|protocol| match protocol.as_str() {
            "https:" => "wss:",
            _ => "ws:",
        });
        let protocol = match protocol {
            Ok(protocol) => protocol,
            Err(_) => return,
        };
        let host = match location.host() {
            Ok(host) => host,
            Err(_) => return,
        };
        match provide_websocket(cx, format!("{protocol}//{host}/ws").as_str()) {
            Ok(_) => (),
            Err(_) => log::error!("Failed to connect to WebSocket!"),
        };
    })();

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/chat-r.css"/>
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
