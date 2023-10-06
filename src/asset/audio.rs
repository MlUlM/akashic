use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::Asset;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug, Asset)]
    pub type AudioAsset;

    #[wasm_bindgen(method)]
    pub fn play(this: &AudioAsset) -> AudioPlayer;

    #[wasm_bindgen(method)]
    pub fn stop(this: &AudioAsset);
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type AudioPlayer;

    #[wasm_bindgen(method, getter)]
    pub fn volume(this: &AudioPlayer) -> f32;

    #[wasm_bindgen(method, js_name = changeVolume)]
    pub fn change_volume(this: &AudioPlayer, volume: f32);

    #[wasm_bindgen(method)]
    pub fn play(this: &AudioPlayer, audio: AudioAsset);

    #[wasm_bindgen(method)]
    pub fn stop(this: &AudioPlayer);
}