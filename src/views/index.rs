use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
};

use crate::components::{
    form::Input,
    todo::{List, ListItem},
};

struct ItemData {
    pub id: u32,
    pub name: String,
    pub complete: bool,
}

pub struct Index {
    link: ComponentLink<Self>,
    current_todo: String,
    items: Vec<ItemData>,
    internal_id: u32,
}

pub enum IndexMsg {
    InputChange(String),
    Keypress(u32),
}

pub enum Keycode {
    Enter = 13
}

fn is_keycode(value: u32, code: Keycode) -> bool { value == code as u32 }

impl Component for Index {
    type Message = IndexMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            current_todo: String::new(),
            items: vec![
                ItemData { name: "Learn Rust".to_string(), id: 1, complete: false },
                ItemData { name: "Learn Yew".to_string(), id: 2, complete: false },
            ],
            internal_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IndexMsg::InputChange(input) => {
                self.current_todo = input;

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
                        }
                    },
                    _ => {}
                }
            },
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        let items = self.render_items();

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
    fn render_items(&self) -> Vec<Html> {
        self.items.iter()
            .map(|litem| {
                let ItemData { name, id, complete } = litem;

                html! {
                    <ListItem
                        key={ *id as i128 }
                        id=id
                        class="todo"
                        item=name
                        complete=complete
                    />
                }
            }).collect::<Vec<Html>>()
    }
}
