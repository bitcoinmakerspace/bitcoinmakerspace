use crate::{
    library::{Route, TitledRoutes, BTCMS_NAME},
    store::{set_drawer, GlobalStore},
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let (_, dispatch) = use_store::<GlobalStore>();
    let route = use_route::<Route>().unwrap_or_default();
    let navigator = use_navigator().unwrap();

    let titled_routes_1: Vec<TitledRoutes> = vec![
        TitledRoutes {
            title: t!("common.about"),
            route: Route::Home,
        },
        TitledRoutes {
            title: t!("acronym.frequently_asked_questions"),
            route: Route::Faq,
        },
        TitledRoutes {
            title: t!("common.contact"),
            route: Route::Contact,
        },
    ];

    let open_drawer: Callback<MouseEvent> = {
        let dispatchcl: Dispatch<GlobalStore> = dispatch.clone();
        Callback::from(move |event: MouseEvent| {
            let disp = dispatchcl.clone();
            event.prevent_default();
            set_drawer(true, disp);
        })
    };

    let click_logo: Callback<MouseEvent> = {
        let route = route.clone();
        if route == Route::Home {
            Callback::from(move |event: MouseEvent| {
                let disp = dispatch.clone();
                event.prevent_default();
                set_drawer(true, disp);
            })
        } else {
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                navigator.push(&Route::Home)
            })
        }
    };

    html! {
        <nav class={classes!("flex", "max-lg:flex-col", "flex-row", "w-full", "max-lg:px-12", "px-12", "max-lg:pt-16", "pt-24", "max-lg:space-y-16", "lg:space-x-16", "lg:items-start")}>
            <div class={classes!("flex", "flex-row", "max-lg:w-full", "w-auto", "justify-between")}>
                <button onclick={click_logo} class={classes!("buttons", "flex", "flex-row")}>
                    <p class={classes!("textbase", "font-op", "font-medium", "max-lg:text-4xl", "text-2xl", "lowercase", "text-start")}>
                        {format!("{}", BTCMS_NAME)}
                    </p>
                </button>
                <div class={classes!("hidden", "max-lg:block", "flex", "flex-row", "-translate-y-1")}>
                    <button onclick={open_drawer} class={classes!("textlt")}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" class="w-16 h-16">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                        </svg>
                    </button>
                </div>
            </div>
            <div class={classes!("flex", "flex-col")}>
                { for titled_routes_1.iter().map(|l| {
                    let class_text_color = if l.route == route { String::from("textli-hl") } else { String::from("textli") };
                    html! {
                        <div class={classes!("flex")}>
                            <Link<Route> to={l.route.clone()}>
                                <p class={classes!("font-op", "font-medium", "max-lg:text-4xl", "text-2xl", "lowercase", "cursor-pointer", class_text_color)}>
                                    {&l.title}
                                </p>
                            </Link<Route>>
                        </div>
                    }
                })}
            </div>
        </nav>
    }
}
