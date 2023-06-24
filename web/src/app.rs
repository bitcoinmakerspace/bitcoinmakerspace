use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    library::{Locale, Route},
    routes::{Home, NotFound},
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::NotFound => html! { <NotFound />},
    }
}

fn get_browser_locale() -> Locale {
    if let Some(language) = gloo::utils::window().navigator().language() {
        Locale::parse_str(language.as_str())
    } else {
        Locale::En
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let locale = get_browser_locale();
    rust_i18n::set_locale(locale.to_string());

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
