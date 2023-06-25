use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    library::{ColorMode, Locale, Route},
    routes::{Contact, Faq, Home, NotFound, Wall},
    store::GlobalStore,
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::Contact => html! { <Contact />},
        Route::Faq => html! { <Faq />},
        Route::Wall => html! { <Wall />},
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

    let (store, _) = use_store::<GlobalStore>();

    use_effect_with_deps(
        move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let element = document.query_selector("html").unwrap().unwrap();
            let html = element.dyn_into::<web_sys::HtmlElement>().unwrap();

            if store.color_mode.is_none() {
                let os_dark = web_sys::window()
                    .unwrap()
                    .match_media("(prefers-color-scheme: dark)")
                    .unwrap()
                    .unwrap();
                if os_dark.matches() {
                    html.class_list().remove_1("light").unwrap();
                    html.class_list().add_1("dark").unwrap();
                } else {
                    html.class_list().remove_1("dark").unwrap();
                    html.class_list().add_1("light").unwrap();
                }
            } else if let Some(ColorMode::Light) = store.color_mode {
                html.class_list().remove_1("dark").unwrap();
                html.class_list().add_1("light").unwrap();
            } else if let Some(ColorMode::Dark) = store.color_mode {
                html.class_list().remove_1("light").unwrap();
                html.class_list().add_1("dark").unwrap();
            }

            || {}
        },
        (),
    );

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
