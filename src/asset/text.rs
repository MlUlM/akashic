use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::Asset;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Asset)]
    pub type TextAsset;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn data(this: &TextAsset) -> String;
}