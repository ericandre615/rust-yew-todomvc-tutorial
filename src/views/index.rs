use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};

use crate::components::todo::{List, ListItem};

pub struct Index {
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        html! {
            <>
                <header class="header">
                    <h1>{ "todos" }</h1>
                </header>
                <section class="main">
                    <List class="todo-list">
                        <ListItem class="todo" item={ "Learn Rust" } />
                        <ListItem class="todo" item={ "Learn Yew" } />
                        <ListItem class="todo" item={ "Check Code" } />
                    </List>
                </section>
            </>
        }
    }
}
