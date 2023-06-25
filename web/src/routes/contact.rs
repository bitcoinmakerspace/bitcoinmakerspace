use crate::{
    components::{Container, PageDetail},
    library::{BTCMS_EMAIL, BTCMS_NOSTR, BTCMS_NPUB},
    store::GlobalStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    let (_, _) = use_store::<GlobalStore>();

    html! {
        <Container>
            <PageDetail>
                <div class={classes!("flex", "flex-col", "w-full", "space-y-12", "max-lg:pr-0", "pr-48 ", "pb-16")}>
                    <div class={classes!("flex", "flex-col", "space-y-6")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-4xl", "text-3xl")}>
                            {format!("{}", t!("common.contact"))}
                        </p>
                    </div>
                    <div class={classes!("flex", "flex-col", "w-full", "justify-center", "items-start", "space-y-2", "max-lg:pr-16")}>
                        <a href={format!("mailto:{}", BTCMS_EMAIL)}>
                            <p class={classes!("textbase", "textli", "font-op", "font-medium", "max-lg:text-3xl", "text-2xl")}>
                                {format!("> {}", BTCMS_EMAIL)}
                            </p>
                        </a>
                        <a href={BTCMS_NOSTR}>
                            <p class={classes!("textbase", "textli", "font-op", "font-medium", "max-lg:text-3xl", "text-2xl", "break-all")}>
                                {format!("> {}", BTCMS_NPUB)}
                            </p>
                        </a>
                    </div>
                </div>
            </PageDetail>
        </Container>
    }
}
