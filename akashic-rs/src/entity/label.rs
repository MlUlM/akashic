use derive_builder::Builder;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EParamSetters, object_2d_parameter, object_e_parameter};

#[object_2d_parameter]
#[object_e_parameter]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Builder, Default, EParamSetters)]
pub struct LabelParameterObject {
    #[builder(setter(into))]
    pub text: String,

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
