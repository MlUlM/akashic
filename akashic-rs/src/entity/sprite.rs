use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{CacheableEntity, EParamSetters, object_e_parameter};

use crate::asset::src::Src;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = g)]
    #[derive(Clone, Debug, crate::entity::AkashicEntity, CacheableEntity)]
    pub type Sprite;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: SpriteParameterObject) -> Sprite;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn local(this: &Sprite) -> String;
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
pub struct SpriteParameterObject {
    #[builder(setter(custom))]
    pub src: JsValue,

    #[wasm_bindgen(js_name = srcWidth)]
    #[builder(setter(into, strip_option), default)]
    pub src_width: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcHeight)]
    #[builder(setter(into, strip_option), default)]
    pub src_height: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcX)]
    #[builder(setter(into, strip_option), default)]
    pub src_x: crate::param::OptionNumber,

    #[wasm_bindgen(js_name = srcY)]
    #[builder(setter(into, strip_option), default)]
    pub src_y: crate::param::OptionNumber,
}


impl SpriteParameterObjectBuilder {
    #[inline]
    pub fn new(
        src: Src
    ) -> Self {
        Self {
            src: Some(src.into()),
            ..SpriteParameterObjectBuilder::empty()
        }
    }


    pub fn src(&mut self, src: Src) -> &mut Self {
        let new = self;
        let src: JsValue = src.into();
        new.src = Some(src);
        new
    }


    #[inline]
    pub fn build(&self) -> SpriteParameterObject {
        self
            .fallible_build()
            .expect("All required fields were initialized")
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
