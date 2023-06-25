use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/faq")]
    Faq,
    #[at("/wall")]
    Wall,
    #[not_found]
    #[at("/404")]
    NotFound,
}
