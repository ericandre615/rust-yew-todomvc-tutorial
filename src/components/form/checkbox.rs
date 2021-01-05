use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Callback,
};

#[derive(Properties, Clone, PartialEq)]
pub struct CheckboxProps {
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<bool>,
}

pub struct Checkbox {
    link: ComponentLink<Self>,
    props: CheckboxProps,
    checked: bool,
}

pub enum CheckboxMsg {
    ToggleChecked,
}

impl Component for Checkbox {
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked;

        Self {
            link,
            props,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CheckboxMsg::ToggleChecked => {
                self.checked = !self.checked;
                self.props.handle_change.emit(self.checked);
            },
        }

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
        let value = self.props.value.clone();
        let classes = self.props.class.clone();

        html! {
            <input
                type="checkbox"
                class=classes
                onclick=self.link.callback(|_| CheckboxMsg::ToggleChecked)
                checked={ self.checked }
                value=value
            />
        }
    }
}
