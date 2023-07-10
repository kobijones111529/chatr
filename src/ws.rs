use crate::chat::message::ServerMessage;
use cfg_if::cfg_if;
use leptos::*;

pub fn create_ws_signal(cx: Scope) -> ReadSignal<Option<ServerMessage>> {
    #[cfg(target_arch = "wasm32")]
    let (get, set) = create_signal(cx, None);

    #[cfg(not(target_arch = "wasm32"))]
    let (get, _) = create_signal(cx, None);

    #[cfg(target_arch = "wasm32")]
    create_ws_signal_wasm32(cx, set);

    get
}

#[cfg(target_arch = "wasm32")]
fn create_ws_signal_wasm32(cx: Scope, set: WriteSignal<Option<ServerMessage>>) {
    use js_sys::{Function, JsString};
    use wasm_bindgen::{prelude::Closure, JsCast};
    use web_sys::MessageEvent;

    let ws = use_context::<ServerWS>(cx);

    match ws {
        Some(ServerWS(ws)) => {
            create_effect(cx, move |_| {
                let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
                    log::info!("Received a message from the server!");
                    let ws_string = event
                        .data()
                        .dyn_into::<JsString>()
                        .unwrap()
                        .as_string()
                        .unwrap();
                    log::info!("Message: {ws_string}");
                    let parsed = serde_json::from_str::<ServerMessage>(&ws_string);
                    match parsed {
                        Ok(parsed) => {
                            log::info!("Parsed: {parsed:?}");
                            set.set(Some(parsed));
                        }
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

type TypeFn<T> = fn(T, String);

pub struct FnStruct<T> {
    pub t: T,
    pub f: TypeFn<T>,
}

impl<T> Clone for FnStruct<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        FnStruct {
            t: self.t,
            f: self.f,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn on_ws_message<T>(cx: Scope, f: FnStruct<T>)
where
    T: Copy + Clone + 'static,
{
    #[cfg(target_arch = "wasm32")]
    on_ws_message_wasm32(cx, f);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn on_ws_message<T>(_cx: Scope, _f: FnStruct<T>)
where
    T: Copy + Clone + 'static,
{
}

#[cfg(target_arch = "wasm32")]
fn on_ws_message_wasm32<T>(cx: Scope, f: FnStruct<T>)
where
    T: Copy + Clone + 'static,
{
    use js_sys::{Function, JsString};
    use wasm_bindgen::{prelude::Closure, JsCast};
    use web_sys::MessageEvent;

    let ws = use_context::<ServerWS>(cx);

    match ws {
        Some(ServerWS(ws)) => create_effect(cx, move |_| {
            let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
                log::info!("Received a message from the server!");
                let ws_string = event
                    .data()
                    .dyn_into::<JsString>()
                    .unwrap()
                    .as_string()
                    .unwrap();
                log::info!("Message: {ws_string}");
                let parsed = serde_json::from_str::<ServerMessage>(&ws_string);
                match parsed {
                    Ok(parsed) => {
                        log::info!("Parsed: {parsed:?}");
                        (f.f)(f.t.clone(), parsed.msg);
                    }
                    Err(err) => {
                        log::error!("Failed to parse: {err:?}");
                    }
                }
            }) as Box<dyn FnMut(_)>);
            let function: &Function = callback.as_ref().unchecked_ref();
            ws.set_onmessage(Some(function));
            callback.forget();
        }),
        None => {
            leptos::error!(r#"No websocket provided at root of app"#);
        }
    }
}

cfg_if! {
if #[cfg(target_arch = "wasm32")] {
    pub fn send_msg(cx: Scope, msg: &str) -> Result<(), ()> {
        use crate::chat::message::ClientMessage;

        let ws = use_context::<ServerWS>(cx);
        match ws {
            Some(ServerWS(ws)) => {
                let msg: ClientMessage = msg.to_string();
                let str = serde_json::to_string(&msg);
                let _ = match str {
                    Ok(str) => ws.send_with_str(str.as_str()),
                    Err(_) => return Err(()),
                };
                Ok(())
            },
            None => Err(()),
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
