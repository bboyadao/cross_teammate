use wasm_bindgen::prelude::*;
use mate_core;

#[wasm_bindgen]
pub struct JsUlid {
    value: String,
}

#[wasm_bindgen]
impl JsUlid {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsUlid {
        let id = mate_core::new_ulid();
        JsUlid { value: id.to_string() }
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(s: &str) -> Result<JsUlid, JsValue> {
        mate_core::ulid_from_string(s)
            .map(|id| JsUlid { value: id.to_string() })
            .map_err(|e| JsValue::from_str(&e))
    }
}
