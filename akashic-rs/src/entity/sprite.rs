use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EParamSetters,  object_2d_parameter, object_e_parameter};

use crate::asset::src::Src;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = g)]
    #[derive(Clone, Debug, crate::entity::AkashicEntity)]
    pub type Sprite;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: SpriteParameterObject) -> Sprite;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn local(this: &Sprite) -> String;
}


#[object_2d_parameter]
#[object_e_parameter]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Builder, Default, EParamSetters)]
pub struct SpriteParameterObject {
    #[builder(setter(custom), default)]
    pub src: JsValue,

    #[wasm_bindgen(js_name = srcWidth)]
    #[builder(default)]
    pub src_width: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcHeight)]
    #[builder(default)]
    pub src_height: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcX)]
    #[builder(default)]
    pub src_x: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcY)]
    #[builder(default)]
    pub src_y: crate::param::OptionNumber,
}


impl SpriteParameterObjectBuilder {
    pub fn src(&mut self, src: Src) -> &mut Self {
        let new = self;
        let src: JsValue = src.into();
        new.src = Some(src);
        new
    }
}


#[allow(clippy::from_over_into)]
impl Into<JsValue> for Src {
    fn into(self) -> JsValue {
        match self {
            Self::Surface(surface) => surface.into(),
            Self::ImageAsset(image_asset) => image_asset.into(),
        }
    }
}
