use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::Src;
use crate::util::into_js_value::macros::into_js_value;

pub mod surface;
pub mod src;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type AssetAccessor;

    #[wasm_bindgen(js_namespace = g, method, js_name = getAllImages)]
    pub fn get_all_images(this: &AssetAccessor, path: String) -> Box<[ImageAsset]>;

    #[wasm_bindgen(js_namespace = g, method, js_name = getImageById)]
    pub fn get_image_by_id(this: &AssetAccessor, asset_id: String) -> ImageAsset;

    #[wasm_bindgen(js_namespace = g, method, js_name = getAllAudios)]
    pub fn get_all_audios(this: &AssetAccessor, path: String) -> Box<[AudioAsset]>;

    #[wasm_bindgen(js_namespace = g, method, js_name = getAudioById)]
    pub fn get_audio_by_id(this: &AssetAccessor, asset_id: String) -> AudioAsset;

    #[wasm_bindgen(js_namespace = g, method, js_name = getTextById)]
    pub fn get_text_by_id(this: &AssetAccessor, asset_id: String) -> TextAsset;
}


impl AssetAccessor {
    #[inline]
    pub fn get_all_images_map(&self, path: impl Into<String>) -> HashMap<String, ImageAsset> {
        self
            .get_all_images(path.into())
            .iter()
            .map(|asset| (asset.id(), asset.clone()))
            .collect()
    }

    #[inline]
    pub fn get_all_audios_map(&self, path: impl Into<String>) -> HashMap<String, AudioAsset> {
        self
            .get_all_audios(path.into())
            .iter()
            .map(|asset| (asset.id(), asset.clone()))
            .collect()
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type ImageAsset;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn id(this: &ImageAsset) -> String;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn path(this: &ImageAsset) -> String;
}

into_js_value!(ImageAsset);



#[allow(clippy::from_over_into)]
impl Into<Src> for ImageAsset {
    #[inline]
    fn into(self) -> Src {
        Src::ImageAsset(self)
    }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type AudioAsset;

    #[derive(Clone, Debug)]
    pub type AudioPlayer;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn id(this: &AudioAsset) -> String;

    #[wasm_bindgen(js_namespace = g, method)]
    pub fn play(this: &AudioAsset) -> AudioPlayer;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn path(this: &AudioAsset) -> String;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type TextAsset;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn id(this: &TextAsset) -> String;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn data(this: &TextAsset) -> String;
}