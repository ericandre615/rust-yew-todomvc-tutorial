#![recursion_limit = "512"]

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use yew;

mod app;
mod views;
mod components;
mod routes;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
