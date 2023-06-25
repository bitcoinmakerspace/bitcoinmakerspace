use crate::{
    components::{Container, PageDetail},
    store::GlobalStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Wall)]
pub fn wall() -> Html {
    let (_, _) = use_store::<GlobalStore>();

    html! {
        <Container>
            <PageDetail>
                <div class={classes!("flex", "flex-col", "w-full", "space-y-12", "max-lg:pr-0", "pr-48 ", "pb-16")}>
                    <div class={classes!("flex", "flex-col", "space-y-6")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-5xl", "text-3xl")}>
                            {format!("Nostr {}", t!("common.wall"))}
                        </p>
                    </div>
                    <div class={classes!("flex", "flex-col", "w-full", "h-[40rem]", "justify-center", "items-center", "bordered", "bordercolor")}>
                        <p class={classes!("textbase", "font-op", "font-medium", "text-xl")}>
                            {format!("{}", t!("wall.title"))}
                        </p>
                    </div>
                </div>
            </PageDetail>
        </Container>
    }
}
