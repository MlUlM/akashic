use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

//
// #[wasm_bindgen(js_namespace=g)]
// extern "C"{
//     #[derive(Debug, Clone)]
//     pub type GlyphArea;
// }


#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GlyphArea {
    pub x: f32,
    pub y: f32,
    pub height: Option<f32>,
    pub width: Option<f32>,

    #[wasm_bindgen(js_name = "offsetX")]
    #[serde(rename = "offsetX")]
    pub offset_x: Option<f32>,

    #[wasm_bindgen(js_name = "offsetY")]
    #[serde(rename = "offsetY")]
    pub offset_y: Option<f32>,

    #[wasm_bindgen(js_name = "advanceWidth")]
    #[serde(rename = "advanceWidth")]
    pub advance_width: Option<f32>,
}