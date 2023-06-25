use crate::{
    components::{Drawer, Footer, Navigation},
    store::GlobalStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
    pub cl: Option<String>,
    pub no_nav: Option<bool>,
    pub no_footer: Option<bool>,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let (store, _) = use_store::<GlobalStore>();

    html! {
        <div class={classes!("flex", "flex-col")}>
            {if !store.drawer_open {
                html! {
                    <div class={classes!("flex", "flex-col", "min-h-screen", "w-full", "max-lg:space-y-16", "space-y-12", "bg-neutral-50", "dark:bg-neutral-800")}>
                        { if props.no_nav == Some(true) {
                            html! {
                                <></>
                            }
                        } else {
                            html! {
                                <Navigation />
                            }
                        }}
                        <div class={classes!("flex", "flex-col", "px-6", props.cl.clone())}>
                            { for props.children.iter() }
                        </div>
                        { if props.no_footer == Some(true) {
                            html! {
                                <></>
                            }
                        } else {
                            html!{
                                <Footer />
                            }
                        }}
                    </div>
                    }
                } else {
                html! {
                    <div class={classes!("flex", "flex-col", "min-h-screen", "p-4", "bg-neutral-50", "dark:bg-neutral-800")}>
                        <Drawer />
                    </div>
                    }
                }
            }
        </div>
    }
}
