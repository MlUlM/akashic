use derive_builder::Builder;

use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EntityObject2D, EParamSetters, object_e_parameter};


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, EntityObject2D)]
    pub type FilledRect;

    #[wasm_bindgen(constructor)]
    pub fn new(param: FilledRectParameter) -> FilledRect;
}


#[non_exhaustive]
#[object_e_parameter]
#[allow(unused_variables)]
#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Builder, EParamSetters)]
pub struct FilledRectParameter {
    #[wasm_bindgen(js_name = cssColor)]
    pub css_color: String,
    pub width: f32,
    pub height: f32,
}