use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::{IntoSrc, Src};

pub mod surface;
pub mod src;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type AssetAccessor;

    #[wasm_bindgen(js_namespace = g, method, js_name = getImageById)]
    pub fn get_image_by_id(this: &AssetAccessor, asset_id: String) -> ImageAsset;

    #[wasm_bindgen(js_namespace = g, method, js_name = getAudioById)]
    pub fn get_audio_by_id(this: &AssetAccessor, asset_id: String) -> AudioAsset;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type ImageAsset;
}


impl IntoSrc for ImageAsset {
    #[inline]
    fn into_src(self) -> Src {
        Src::ImageAsset(self)
    }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type AudioAsset;

    #[derive(Clone, Debug)]
    pub type AudioPlayer;

    #[wasm_bindgen(js_namespace = g, method)]
    pub fn play(this: &AudioAsset) -> AudioPlayer;
}
