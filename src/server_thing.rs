use leptos::*;

#[server(ServerThing, "/api")]
pub async fn do_thing() -> Result<String, ServerFnError> {
    Ok("Hi".to_string())
}
