use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    Callback,
};

#[derive(Properties, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_click: Callback<bool>,
}

pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

pub enum ButtonMsg {
    Clicked,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ButtonMsg::Clicked => {
                self.props.handle_click.emit(true);
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let children = self.props.children.clone();

        html! {
            <button
                class=format!("btn {}", classes)
                onclick=self.link.callback(|_| ButtonMsg::Clicked)
            >
                { children }
            </button>
        }
    }
}
