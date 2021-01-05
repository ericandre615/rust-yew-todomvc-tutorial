use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    InputData,
    Callback,
};

#[derive(Properties, Clone, Debug)]
pub struct InputProps {
    #[prop_or(String::new())]
    pub initial_value: String,
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<String>,
}

pub struct Input {
    link: ComponentLink<Self>,
    props: InputProps,
    value: String,
}

pub enum InputMsg {
    UpdateValue(InputData)
}

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = if !props.value.is_empty() {
            props.value.clone()
        } else {
            props.initial_value.clone()
        };

        Self {
            link,
            props,
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InputMsg::UpdateValue(change) => {
                self.value = change.value.clone();
                self.props.handle_change.emit(self.value.clone());
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.props.value = props.value.clone();
            self.value = props.value;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let label = self.props.label.clone();
        let placeholder = self.props.placeholder.clone();
        let classes = self.props.class.clone();

        html! {
            <div class="form-group input-group">
                <label>{ label }</label>
                <input
                    type="text"
                    class=classes
                    oninput=self.link.callback(|v: InputData| InputMsg::UpdateValue(v))
                    value={ self.value.clone() }
                    placeholder={ placeholder }
                />
            </div>
        }
    }
}
