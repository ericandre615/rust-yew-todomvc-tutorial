use web_sys::{Storage};
use wasm_bindgen::{JsValue};
use serde_json::{Value};

pub fn get_session(key: &str) -> Result<Value, String> {
    let window = web_sys::window().expect("Window Error");
    let storage = window.session_storage().unwrap().expect("SessionStorage not supported");
    let data_str = storage.get_item(key).unwrap().unwrap_or("[]".to_string());
    let data = serde_json::from_str(&data_str).unwrap_or(Value::Array(Vec::new()));

    Ok(data)
}

pub fn set_session(key: &str, value: &str) -> Result<(), String> {
    let window = web_sys::window().expect("Window Error");
    let storage = window.session_storage().unwrap().expect("SessionStorage not supported");
    
    storage.set_item(key, value);

    Ok(())
}

pub fn clear_session() -> Result<(), String> {
    let window = web_sys::window().expect("Window Error");
    let storage = window.session_storage().unwrap().expect("SessionStorage not supported");

    storage.clear();

    Ok(())
}
