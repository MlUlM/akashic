use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EntityObject2D, object_e_parameter};

use crate::asset::src::Src;
use crate::util::into_js_value::IntoJsValue;

#[wasm_bindgen]
extern "C" {
    /// 画面を描画するエンティティを表します。
    ///
    /// これはアカシックの[`Sprite`](https://akashic-games.github.io/akashic-engine/v3/classes/Sprite.html)と同様のものです。
    #[wasm_bindgen(js_namespace = g)]
    #[derive(Clone, Debug, EntityObject2D)]
    pub type Sprite;


    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: SpriteParam) -> Sprite;
}


#[non_exhaustive]
#[object_e_parameter]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Builder)]
#[builder(
name="SpriteBuilder",
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct SpriteParam {
    #[builder(setter(custom))]
    pub src: JsValue,

    #[wasm_bindgen(js_name = srcWidth)]
    #[builder(setter(into, strip_option), default)]
    pub src_width: crate::option_number::OptionNumber,

    #[wasm_bindgen(js_name = srcHeight)]
    #[builder(setter(into, strip_option), default)]
    pub src_height: crate::option_number::OptionNumber,

    #[wasm_bindgen(js_name = srcX)]
    #[builder(setter(into, strip_option), default)]
    pub src_x: crate::option_number::OptionNumber,

    #[wasm_bindgen(js_name = srcY)]
    #[builder(setter(into, strip_option), default)]
    pub src_y: crate::option_number::OptionNumber,
}


impl SpriteBuilder {
    #[inline]
    pub fn new(
        src: impl Into<Src>
    ) -> Self {
        Self {
            src: Some(src.into().into()),
            ..SpriteBuilder::empty()
        }
    }


    pub fn src(&mut self, src: Src) -> &mut Self {
        let new = self;
        let src: JsValue = src.into();
        new.src = Some(src);
        new
    }


    #[inline]
    pub fn build(&self) -> Sprite {
        Sprite::new(self.fallible_build().unwrap())
    }
}


#[allow(clippy::from_over_into)]
impl Into<JsValue> for Src {
    fn into(self) -> JsValue {
        match self {
            Self::Surface(surface) => surface.into_js_value(),
            Self::ImageAsset(image_asset) => image_asset.into_js_value(),
        }
    }
}
