use yew::prelude::*;
use yew_router::prelude::*;

use crate::library::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = Callback::from(move |_| navigator.replace(&Route::Home));

    html! {
        <div>
            <div class={classes!("flex", "flex-col", "w-full", "space-y-12", "pr-48", "pb-16")}>
                <div class={classes!("flex", "flex-col", "space-y-6")}>
                    <div class={classes!("flex", "flex-row")}>
                        <button onclick={go_home}>
                            <p class={classes!("textbase", "font-op", "font-semibold", "text-xl")}>
                                {format!("<-")}
                            </p>
                        </button>
                    </div>
                    <p class={classes!("textbase", "font-op", "font-semibold", "text-xl")}>
                        {format!("{} The requested page could not be found.", 404)}
                    </p>
                </div>
            </div>
        </div>
    }
}
