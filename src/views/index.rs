use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Properties,
};
use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::components::{
    form::Input,
    todo::{List, ListItem},
};
use crate::app::AppFilter;
use crate::api::session;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct ItemData {
    pub id: u32,
    pub name: String,
    pub complete: bool,
}

#[derive(Properties, Clone)]
pub struct IndexProps {
    pub filter: AppFilter,
}

pub struct Index {
    link: ComponentLink<Self>,
    props: IndexProps,
    current_todo: String,
    items: Vec<ItemData>,
    internal_id: u32,
}

pub enum IndexMsg {
    InputChange(String),
    Keypress(u32),
    ToggleComplete(u32),
    RemoveItem(u32),
}

pub enum Keycode {
    Enter = 13
}

fn is_keycode(value: u32, code: Keycode) -> bool { value == code as u32 }

impl Component for Index {
    type Message = IndexMsg;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = session::get_session(&"items").unwrap();
        let items: Vec<ItemData> = serde_json::from_value(storage).unwrap_or(Vec::new());
        let last_id = Index::get_last_id(&items);

        Self {
            link,
            props,
            current_todo: String::new(),
            items,
            internal_id: last_id,
        }
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IndexMsg::InputChange(input) => {
                self.current_todo = input;

            },
            IndexMsg::ToggleComplete(item_id) => {
                for item in &mut self.items {
                    if item.id == item_id {
                        item.complete = !item.complete;
                    }
                }

                self.store_items();
            },
            IndexMsg::RemoveItem(item_id) => {
                self.items.retain(|item| item.id != item_id);
                self.store_items();
            },
            IndexMsg::Keypress(keycode) => {
                match keycode {
                    _ if is_keycode(keycode, Keycode::Enter) => {
                        let name = self.current_todo.clone();
                        self.current_todo = String::new();

                        if !name.is_empty() {
                            self.items.push(ItemData {
                                id: self.internal_id,
                                name,
                                complete: false,
                            });

                            self.internal_id += 1;
                            self.store_items();
                        }
                    },
                    _ => {}
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.filter != props.filter {
            self.props.filter = props.filter;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let items = self.render_items(&self.props.filter);

        html! {
            <>
                <header
                    class="header"
                    onkeypress=self.link.callback(|e: KeyboardEvent| IndexMsg::Keypress(e.key_code()))
                >
                    <h1>{ "todos" }</h1>
                    <Input
                        class="new-todo"
                        value=self.current_todo.clone()
                        placeholder="What needs to be done?"
                        handle_change=self.link.callback(IndexMsg::InputChange)
                    />
                </header>
                <section class="main">
                    <List class="todo-list">
                        { items }
                    </List>
                </section>
            </>
        }
    }
}

impl Index {
    fn store_items(&self) {
        let json = serde_json::to_string(&self.items);

        match json {
            Ok(json_str) => {
                session::set_session(&"items", &json_str);
            },
            Err(_) => {},
        }
    }

    fn get_last_id(items: &Vec<ItemData>) -> u32 {
        let max_item = items.iter().max();

        match max_item {
            Some(max) => { max.id + 1 },
            None => 0,
        }
    }

    fn render_items(&self, filter: &AppFilter) -> Vec<Html> {
        self.items.iter()
            .filter(|item| {
                match filter {
                    AppFilter::Active => !item.complete,
                    AppFilter::Complete => item.complete,
                    AppFilter::All => true,
                }
            })
            .map(|litem| {
                let ItemData { name, id, complete } = litem;

                html! {
                    <ListItem
                        key={ *id as i128 }
                        id=id
                        class="todo"
                        item=name
                        complete=complete
                        handle_complete=self.link.callback(IndexMsg::ToggleComplete)
                        handle_remove=self.link.callback(IndexMsg::RemoveItem)
                    />
                }
            }).collect::<Vec<Html>>()
    }
}
