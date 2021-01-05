use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Callback,
};

use crate::components::{Button, form::Checkbox};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ListItemProps {
    #[prop_or(0)]
    pub id: u32,
    #[prop_or(String::new())]
    pub item: String,
    #[prop_or(false)]
    pub complete: bool,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_remove: Callback<u32>,
    #[prop_or(Callback::noop())]
    pub handle_complete: Callback<u32>,
}

pub struct ListItem {
    link: ComponentLink<Self>,
    props: ListItemProps,
}

pub enum ListItemMsg {
    ToggleComplete,
    Clicked(bool),
}

impl Component for ListItem {
    type Message = ListItemMsg;
    type Properties = ListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ListItemMsg::ToggleComplete => {
                self.props.handle_complete.emit(self.props.id);
            },
            ListItemMsg::Clicked(clicked) => {
                self.props.handle_remove.emit(self.props.id);
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
        let id = self.props.id;
        let item = self.props.item.clone();
        let classes = self.props.class.clone();
        let completed = if self.props.complete { "completed" } else { "" };

        html! {
            <li class=format!("list-item {} {}", classes, completed)>
                <div class="view" id=id>
                    <Checkbox
                        class="toggle"
                        value=item.clone()
                        checked=self.props.complete
                        handle_change=self.link.callback(|_| ListItemMsg::ToggleComplete)
                    />
                    
                    <label><span class="list-item">{ item }</span></label>
                    <Button class="destroy" handle_click=self.link.callback(ListItemMsg::Clicked) />
                </div>
            </li>
        }
    }
}
