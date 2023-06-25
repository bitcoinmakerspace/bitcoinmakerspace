use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PageDetail)]
pub fn page_detail(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "max-lg:min-h-screen", "lg:min-h-[24rem]", "w-full", "max-lg:px-6", "lg:pl-[19rem]", "pt-8")}>
            { for props.children.iter() }
        </div>
    }
}
