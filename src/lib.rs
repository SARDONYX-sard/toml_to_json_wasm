use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn toml_str_to_json(data: &str) -> JsValue {
    let json: Value = toml::from_str(data).unwrap();
    json.to_string().into()
}

#[test]
fn debug_toml_to_json() {
    const DATA: &str = include_str!("../asset/Cargo.toml");
    let json: Value = toml::from_str(DATA).unwrap();
    println!("{json}");
}
