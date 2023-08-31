use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::{AkashicEntity};


use crate::scene::Scene;


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, AkashicEntity, Debug)]
    pub type FilledRect;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: FilledRectParameter) -> FilledRect;
}


#[wasm_bindgen(getter_with_clone)]
pub struct FilledRectParameter {
    pub scene: Scene,
    #[wasm_bindgen(js_name = cssColor)]
    pub css_color: String,
    pub width: f32,
    pub height: f32,
    pub touchable: bool,
}