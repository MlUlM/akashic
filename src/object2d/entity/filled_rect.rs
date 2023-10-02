use derive_builder::Builder;

use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EntityObject2D, entity_params};


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, EntityObject2D)]
    pub type FilledRect;

    #[wasm_bindgen(constructor)]
    pub fn new(param: FilledRectParam) -> FilledRect;

    #[wasm_bindgen(method, getter, js_name=cssColor)]
    pub fn css_color(this: &FilledRect) -> String;

    #[wasm_bindgen(method, setter, js_name=cssColor)]
    pub fn set_css_color(this: &FilledRect, css_color: String);
}


#[non_exhaustive]
#[entity_params]
#[wasm_bindgen(getter_with_clone)]
#[derive(Builder)]
#[builder(
name="FilledRectBuilder",
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct FilledRectParam {
    #[wasm_bindgen(js_name = cssColor)]
    pub css_color: String,
    pub width: f32,
    pub height: f32,
}


impl FilledRectBuilder {
    pub fn new(css_color: impl Into<String>, width: f32, height: f32) -> Self{
        Self{
            css_color: Some(css_color.into()),
            width: Some(width),
            height: Some(height),
            ..FilledRectBuilder::empty()
        }
    }


    #[inline]
    pub fn build(&self) -> FilledRect {
        FilledRect::new(self.fallible_build().unwrap())
    }
}