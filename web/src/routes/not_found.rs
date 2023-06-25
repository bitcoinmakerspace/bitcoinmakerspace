use crate::{components::Container, library::Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = Callback::from(move |_| navigator.replace(&Route::Home));

    html! {
        <Container cl={"max-lg:pt-12 pt-4 max-lg:px-8 pl-64 min-h-screen"}>
            <div class={classes!("flex", "max-lg:flex-col", "flex-row", "w-full", "max-lg:space-y-12", "space-x-6", "pl-16", "pb-16")}>
                <div class={classes!("flex", "flex-row")}>
                    <button onclick={go_home} class={classes!("buttons")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "text-xl")}>
                            {format!("<-")}
                        </p>
                    </button>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <p class={classes!("textbase", "font-op", "font-semibold", "text-xl")}>
                        {format!("(404) {}", t!("not_found.title"))}
                    </p>
                </div>
            </div>
        </Container>
    }
}
