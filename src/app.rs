use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};

use crate::routes::router;
use crate::components::Filters;

#[derive(Copy, Clone, PartialEq)]
pub enum AppFilter {
    All,
    Active,
    Complete,
}

pub struct App {
    link: ComponentLink<Self>,
    item_count: u32,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            item_count: 0,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender { true }

    fn view(&self) -> Html {
        html! {
            <div id="todo-app" class="todomvc-wrapper">
                <section class="todoapp">
                    { router() }
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.item_count }</strong>
                            { " item(s) left" }
                        </span>
                        <Filters />
                    </footer>
                </section>
                <footer class="info" />
            </div>
        }
    }
}
