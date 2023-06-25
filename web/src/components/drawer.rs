use crate::{
    library::{Route, TitledRoutes},
    store::{set_drawer, GlobalStore},
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Drawer)]
pub fn drawer() -> Html {
    let (_, dispatch) = use_store::<GlobalStore>();

    let close_drawer: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
        let disp = dispatch.clone();
        event.prevent_default();
        set_drawer(false, disp);
    });

    let titled_routes_1: Vec<TitledRoutes> = vec![
        TitledRoutes {
            title: t!("drawer.control_grid.1.links.1.title"),
            route: Route::Home,
        },
        TitledRoutes {
            title: t!("drawer.control_grid.1.links.2.title"),
            route: Route::Home,
        },
    ];

    let titled_routes_2: Vec<TitledRoutes> = vec![
        TitledRoutes {
            title: t!("drawer.control_grid.2.links.1.title"),
            route: Route::Home,
        },
        TitledRoutes {
            title: t!("drawer.control_grid.2.links.2.title"),
            route: Route::Home,
        },
    ];

    let titled_routes_3: Vec<TitledRoutes> = vec![
        TitledRoutes {
            title: t!("drawer.control_grid.3.links.1.title"),
            route: Route::Contact,
        },
        TitledRoutes {
            title: t!("drawer.control_grid.3.links.2.title"),
            route: Route::Contact,
        },
    ];

    let titled_routes: Vec<Vec<TitledRoutes>> =
        vec![titled_routes_1, titled_routes_2, titled_routes_3];

    html! {
        <div class={classes!("flex", "flex-col", "min-h-screen", "w-auto", "p-8", "max-lg:space-y-12", "space-y-2")}>
            <div class={classes!("flex", "flex-row", "w-full", "pt-4", "justify-end", "items-center")}>
                <button onclick={close_drawer.clone()} class={classes!("textlt")}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="max-lg:w-16 max-lg:h-16 w-8 h-8">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
            <div class={classes!("grid", "grid-cols-3", "max-lg:px-4", "px-12")}>
                { for (0..3).map(|li| {
                    html! {
                        <div class={classes!("max-lg:col-span-3", "col-span-1", "flex", "flex-col", "py-12", "space-y-4")}>
                            <p class={classes!("textbase", "font-dina", "font-regular","max-lg:text-5xl", "text-3xl", "underline", "underline-offset-4", "capitalize")}>
                                {format!("{}", t!(&format!("drawer.control_grid.{}.title", li + 1)))}
                            </p>
                            <div class={classes!("flex", "flex-col", "max-lg:space-y-2", "space-y-4", "lg:pr-12")}>
                                { for titled_routes.get(li).map(|li_route| {
                                    li_route.iter().map(|li_link| {
                                        html! {
                                            <div class={classes!("flex", "flex-row", "w-full")}>
                                                <button onclick={close_drawer.clone()}>
                                                    <Link<Route> to={li_link.route.clone()}>
                                                        <p class={classes!("textli", "font-op", "font-medium", "max-lg:text-5xl", "text-2xl", "capitalize", "cursor-pointer", "text-start")}>
                                                            {&li_link.title}
                                                        </p>
                                                    </Link<Route>>
                                                </button>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }) }
                            </div>
                        </div>
                    }
                })}
            </div>
       </div>
    }
}
