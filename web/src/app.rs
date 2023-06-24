use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("textbase", "font-op")}>{"bitcoin makerspace"}</div>
    }
}
