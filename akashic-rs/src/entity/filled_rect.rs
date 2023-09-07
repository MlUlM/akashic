use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{AkashicEntity, EParamSetters, Object2dParamSetters, object_2d_parameter, object_e_parameter};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, AkashicEntity, Debug)]
    pub type FilledRect;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: FilledRectParameter) -> FilledRect;
}


#[object_2d_parameter]
#[object_e_parameter]
#[allow(unused_variables)]
#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Builder, EParamSetters, Object2dParamSetters)]
pub struct FilledRectParameter {
    #[wasm_bindgen(js_name = cssColor)]
    pub css_color: String,
    pub width: f32,
    pub height: f32,
}