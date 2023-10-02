
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "'normal' | 'bold'")]
    pub type FontWeightString;
}


impl FontWeightString {
    #[inline]
    pub fn normal() -> Self {
        JsValue::from_str("normal").unchecked_into()
    }

    #[inline]
    pub fn bold() -> Self {
        JsValue::from_str("bold").unchecked_into()
    }
}