use yew::prelude::*;

use crate::{components::ColorModeSwitch, library::BTCMS_NAME};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("backgrounds", "h-screen", "w-full", "flex", "flex-row", "p-4", "space-x-4")}>
            <p class={classes!("textbase", "font-op", "font-medium", "text-xl", "lowercase")}>
                {format!("{}", BTCMS_NAME)}
            </p>
            <ColorModeSwitch />
        </div>
    }
}
