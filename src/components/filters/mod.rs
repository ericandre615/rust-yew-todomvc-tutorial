use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
};

use yew_router::components::{RouterAnchor};

use crate::app::AppFilter;
use crate::routes::AppRoute;

#[derive(Properties, Clone, PartialEq)]
pub struct FiltersProps {
    #[prop_or(AppFilter::All)]
    active: AppFilter,
}

pub struct Filters {
    link: ComponentLink<Self>,
    props: FiltersProps,
}

type RouteFilter = RouterAnchor<AppRoute>;

impl Component for Filters {
    type Message = ();
    type Properties = FiltersProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let active = self.props.active;

        html! {
            <ul class="filters">
                <li><RouteFilter route=AppRoute::Index>{ "All" }</RouteFilter></li>
                <li><RouteFilter route=AppRoute::Active>{ "Active" }</RouteFilter></li>
                <li><RouteFilter route=AppRoute::Complete>{ "Complete" }</RouteFilter></li>
            </ul>
        }
    }
}
