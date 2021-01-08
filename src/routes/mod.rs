use yew_router::{Switch, router::Router};
use yew::prelude::{Html, html};

use crate::app::AppFilter;
use crate::views::Index;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/complete"]
    Complete,
    #[to = "/active"]
    Active,
    #[to = "/"]
    Index,
}

pub fn router() -> Html {
    html! {
        <Router<AppRoute, ()>
            render = Router::render(|route: AppRoute| {
                match route {
                    AppRoute::Active => html! { <Index filter=AppFilter::Active /> },
                    AppRoute::Complete => html! { <Index filter=AppFilter::Complete /> },
                    AppRoute::Index => html! { <Index filter=AppFilter::All /> },
                }
            })
        />
    }
}
