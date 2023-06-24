use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    library::Route,
    routes::{Home, NotFound},
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::NotFound => html! { <NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
