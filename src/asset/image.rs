use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::Asset;
use crate::asset::src::Src;
use crate::util::into_js_value::macros::into_js_value;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Asset)]
    pub type ImageAsset;
}

into_js_value!(ImageAsset);


#[allow(clippy::from_over_into)]
impl Into<Src> for ImageAsset {
    #[inline]
    fn into(self) -> Src {
        Src::ImageAsset(self)
    }
}