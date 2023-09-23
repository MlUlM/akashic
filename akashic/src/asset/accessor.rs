use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{AssetAccessor, expand_asset_accessible_traits};

use crate::asset::audio::AudioAsset;
use crate::asset::image::ImageAsset;
use crate::asset::text::TextAsset;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, AssetAccessor)]
    pub type AssetAccessor;
}


expand_asset_accessible_traits!();
