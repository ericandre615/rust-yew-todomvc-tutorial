use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender { true }

    fn view(&self) -> Html {
        html! {
            <h1>{ "Hello, Yew!" }</h1>
        }
    }
}
