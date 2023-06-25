use crate::{
    components::{Container, PageDetail},
    library::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Container>
            <PageDetail>
                <div class={classes!("flex", "flex-col", "w-full", "max-lg:space-y-24", "space-y-12", "max-lg:pr-4", "pr-48", "pb-16")}>
                    <div class={classes!("flex", "flex-col", "max-lg:space-y-16", "space-y-6")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-4xl", "text-3xl")}>
                            {format!("{}", t!("about.title"))}
                        </p>
                        <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "text-justify", "leading-relaxed")}>
                            {format!("{}", t!("about.summary"))}
                        </p>
                    </div>
                    <div class={classes!("flex", "flex-col", "max-lg:space-y-12", "space-y-6")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-4xl", "text-3xl")}>
                            {format!("{}", t!("about.page_content.title"))}
                        </p>
                        { for (0..4).map(|li| {
                            html! {
                                <div class={classes!("flex", "flex-col", "max-lg:space-y-4", "space-y-1")}>
                                    <p class={classes!("textbase", "font-dina", "underline", "font-medium","max-lg:text-3xl", "text-2xl", "text-justify")}>
                                        {format!("{}:", t!(&format!("about.page_content.reasons.{}.title", li + 1)))}
                                    </p>
                                    <p class={classes!("textbase", "font-dina", "font-regular","max-lg:text-3xl", "text-2xl", "text-justify")}>
                                        {format!(" {}", t!(&format!("about.page_content.reasons.{}.summary", li + 1)))}
                                    </p>
                                </div>
                            }
                        })}
                    </div>
                    <div class={classes!("flex", "flex-col", "max-lg:space-y-16", "space-y-6")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-4xl", "text-3xl", "capitalize")}>
                            {format!("{}", t!("common.join_us"))}
                        </p>
                        <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "text-justify", "leading-relaxed")}>
                            {format!("{}", t!("about.conclusion.1"))}
                        </p>
                        <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "text-start", "leading-relaxed")}>
                            {format!("-- {}", t!("about.conclusion.2"))}
                        </p>

                        <div class={classes!("flex", "flex-row", "w-full", "py-4")}>
                            <Link<Route> to={Route::Wall}>
                                <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "hover:underline", "underline-offset-4")}>
                                    {format!(">> {}", t!("about.conclusion.3"))}
                                </p>
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </PageDetail>
        </Container>
    }
}
