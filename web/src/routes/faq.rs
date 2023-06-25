use crate::components::{Container, PageDetail};
use yew::prelude::*;

#[function_component(Faq)]
pub fn faq() -> Html {
    html! {
        <Container>
            <PageDetail>
                <div class={classes!("flex", "flex-col", "w-full", "space-y-12", "max-lg:pr-0", "pr-48", "pb-16")}>
                    <div class={classes!("flex", "flex-col", "max-lg:space-y-16", "space-y-8")}>
                        <p class={classes!("textbase", "font-op", "font-semibold", "max-lg:text-4xl", "text-3xl")}>
                            {format!("{}", t!("faq.title"))}
                        </p>
                        { for (0..6).map(|li| {
                            html! {
                                <div class={classes!("flex", "flex-col", "w-full", "max-lg:space-y-8", "space-y-4", "py-2")}>
                                    <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "text-justify", "leading-relaxed")}>
                                        {format!("{}. {}", li + 1, t!(&format!("faq.page_content.{}.question", li + 1)))}
                                    </p>
                                    <p class={classes!("textbase", "font-dina", "font-regular", "max-lg:text-3xl", "text-2xl", "text-justify", "leading-relaxed")}>
                                        {format!("{}", t!(&format!("faq.page_content.{}.answer", li + 1)))}
                                    </p>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </PageDetail>
        </Container>
    }
}
