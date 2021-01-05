use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
}

pub struct List {
    link: ComponentLink<Self>,
    props: ListProps,
}

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let classes = self.props.class.clone();
        let children = self.props.children.clone();

        html! {
            <ul class=format!("list {}", classes)>
                { children }
            </ul>
        }
    }
}
