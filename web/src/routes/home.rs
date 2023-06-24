use yew::prelude::*;

use crate::library::BTCMS_NAME;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <p class={classes!("textbase", "font-op", "font-medium", "text-xl", "lowercase")}>
                {format!("{}", BTCMS_NAME)}
            </p>
        </div>
    }
}
