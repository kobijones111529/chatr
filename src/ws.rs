use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg: String,
}

pub fn use_ws(cx: Scope) -> () {
    cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::{prelude::Closure, JsCast};
        use web_sys::MessageEvent;
        use js_sys::{Function, JsString};

        let ws = use_context::<ServerWS>(cx);

        match ws {
            Some(ServerWS(ws)) => {
                create_effect(cx, move |_| {
                    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
                        log::info!("Received a message from the server!");
                        let ws_string = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                        log::info!("Message: {ws_string}");
                        let parsed = serde_json::from_str::<Message>(&ws_string);
                        match parsed {
                            Ok(parsed) => {
                                log::info!("Parsed: {parsed:?}");
                            },
                            Err(err) => {
                                log::error!("Failed to parse: {err:?}");
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    let function: &Function = callback.as_ref().unchecked_ref();

                    ws.set_onmessage(Some(function));
                    callback.forget();
                });
            }
            None => {
                leptos::error!(r#"No websocket provided at root of app"#);
            }
        }
    }
    }
}

cfg_if! {
if #[cfg(target_arch = "wasm32")] {
    pub fn send_msg(cx: Scope, msg: &str) {
        let ws = use_context::<ServerWS>(cx);
        match ws {
            Some(ServerWS(ws)) => {
                let _ = ws.send_with_str(format!("{{\"msg\": \"{}\"}}", msg).as_str());
            }
            None => ()
        }
    }
} else {
    pub fn send_msg(_cx: Scope, _msg: &str) { }
}
}

cfg_if! {
if #[cfg(target_arch = "wasm32")] {
    use web_sys::WebSocket;
    use wasm_bindgen::JsValue;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ServerWS(WebSocket);

    pub fn provide_websocket(cx: Scope, url: &str) -> Result<(), JsValue> {
        if use_context::<ServerWS>(cx).is_none() {
            let ws = WebSocket::new(url)?;
            log::info!("Created WS: {ws:?}");
            provide_context(cx, ServerWS(ws));
        }

        Ok(())
    }
} else {
    pub fn provide_websocket(_cx: Scope, _url: &str) -> Result<(), ()> {
        Ok(())
    }
}
}
