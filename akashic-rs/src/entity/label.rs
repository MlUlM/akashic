use derive_builder::Builder;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::{CacheableEntity, EParamSetters, object_e_parameter};
use crate::font::Font;


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, crate::entity::AkashicEntity, CacheableEntity)]
    #[wasm_bindgen(js_namespace = g, js_name = Label)]
    pub type Label;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: LabelParameterObject) -> Label;
}


#[non_exhaustive]
#[object_e_parameter]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Builder, EParamSetters)]
#[builder(
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct LabelParameterObject {
    #[builder(setter(into))]
    pub text: String,

    #[builder(setter(into))]
    pub font: Font,

    #[wasm_bindgen(js_name = fontSize)]
    #[builder(setter(into, strip_option), default)]
    pub font_size: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = textAlign)]
    #[builder(setter(into, strip_option), default)]
    pub text_align: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = maxWidth)]
    #[builder(setter(into, strip_option), default)]
    pub max_width: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = widthAutoAdjust)]
    #[builder(setter(into, strip_option), default)]
    pub width_auto_adjust: Option<bool>,

    #[wasm_bindgen(js_name = textColor)]
    #[builder(setter(into, strip_option), default)]
    pub text_color: Option<String>,
}


impl LabelParameterObjectBuilder {
    #[inline]
    pub fn new(
        text: impl Into<String>,
        font: impl Into<Font>,
    ) -> Self {
        Self {
            text: Some(text.into()),
            font: Some(font.into()),
            ..LabelParameterObjectBuilder::empty()
        }
    }

    #[inline]
    pub fn build(&self) -> LabelParameterObject {
        self
            .fallible_build()
            .expect("All required fields were initialized")
    }
}