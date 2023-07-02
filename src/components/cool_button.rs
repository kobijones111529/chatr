use crate::{server_thing::ServerThing, ws::use_ws, ws::send_msg};
use leptos::*;
use leptos_router::*;

#[component]
pub fn CoolButton(cx: Scope, offset: ReadSignal<i32>) -> impl IntoView {
    let (count, _) = create_signal(cx, 0);

    let do_thing_action = create_server_action::<ServerThing>(cx);

    create_effect(cx, move |_| {
        let str = do_thing_action.value().get();
        str.map(|val| match val {
            Ok(str) => {
                log::info!("{}", str);
            }
            Err(_) => ()
        })
    });

    use_ws(cx);

    view! {cx,
        <ActionForm action=do_thing_action>
            <button type="submit">"Submit"</button>
        </ActionForm>
        <p>"Hi"</p>
        <button on:click=move |_| {
            let _ = send_msg(cx, "{\"str\": \"Hi\"}");
        }>"Click"</button>
        <p>"Count is " {move || count.get() + offset.get()}</p>
    }
}
