use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string | string[]")]
    #[derive(Clone, Debug)]
    pub type FontFamily;
}


impl FontFamily {
    #[inline]
    pub fn new(font: impl Into<String>) -> Self {
        let font: String = font.into();
        JsValue::from(font).unchecked_into()
    }


    #[inline]
    pub fn fonts<'a>(fonts: impl Iterator<Item=&'a str>) -> Self {
        let fonts = fonts.map(JsValue::from_str).collect::<Array>();
        JsValue::from(fonts).unchecked_into()
    }
}