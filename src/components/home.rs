use crate::{
    chat::message::{Sender, ServerMessage},
    ws::{create_ws_signal, send_msg},
};
use leptos::*;
use uuid::Uuid;
use web_sys::SubmitEvent;

#[derive(Debug, Clone)]
enum Message {
    Me(String),
    Server(ServerMessage),
}

/// Renders the home page of your application.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    create_ws_signal(cx);

    let last_message = create_ws_signal(cx);
    let (messages, set_messages) = create_signal(cx, Vec::<(Uuid, Message)>::new());

    create_effect(cx, move |_| match last_message.get() {
        Some(message) => {
            set_messages.update(move |messages| {
                (*messages).push((Uuid::new_v4(), Message::Server(message)));
            });
        }
        None => (),
    });

    let (message_input, set_message_input) = create_signal(cx, "".to_owned());

    let send_message = move |ev: SubmitEvent| {
        ev.prevent_default();

        let msg = message_input.get();

        let _ = send_msg(cx, msg.as_str());
        set_message_input.set("".to_owned());

        set_messages.update(move |messages| {
            (*messages).push((Uuid::new_v4(), Message::Me(msg)));
        });
    };

    view! { cx,
        <div class="chat__outer-container">
            <div class="chat__container">
            <ol class="chat">
                <For
                    each=move || messages.get()
                    key=move |message| message.0
                    view=move |cx, (_, message)| {
                        view! { cx,
                            {
                                move || {
                                    let message = message.clone();
                                match message {
                                Message::Me(msg) => view!{cx,
                                    <li class="chat-message__container chat-message__container--me">
                                    <p class="chat-message__message chat-message__message--me">{
                                        msg
                                    }</p>
                                    </li>
                                }.into_view(cx),
                                Message::Server(message) => {
                                    view! {cx,
                                        <li class="chat-message__container">
                                    <p class="chat-message__sender">{move || {
                                        let sender = &message.sender;
                                        match sender {
                                            Some(sender) => match sender {
                                                Sender::Anonymous => "Anonymous".to_owned(),
                                                Sender::User { name } => name.value().to_owned(),
                                            },
                                            None => "Server".to_owned(),
                                        }
                                    }}</p>
                                    <p class="chat-message__message chat-message__message--server">{move || {
                                        let msg = &message.msg;
                                        msg.to_owned()
                                    }}</p>
                                    </li>
                                }}.into_view(cx)
                            }}}
                            
                    }}
                />
            </ol>
            </div>
        </div>

        <form on:submit=send_message>
            <input
                placeholder="Hello, World?"
                prop:value=message_input
                on:input=move |ev| {
                    set_message_input.set(event_target_value(&ev));
                }
            />
            <button type="submit" disabled=move || message_input.get() == "">
                "Send"
            </button>
        </form>
    }
}
