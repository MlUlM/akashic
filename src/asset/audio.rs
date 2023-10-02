use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::Asset;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Asset)]
    pub type AudioAsset;

    #[derive(Clone, Debug)]
    pub type AudioPlayer;

    #[wasm_bindgen(js_namespace = g, method)]
    pub fn play(this: &AudioAsset) -> AudioPlayer;
}
