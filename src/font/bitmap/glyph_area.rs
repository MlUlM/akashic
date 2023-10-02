use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[non_exhaustive]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GlyphArea {
    pub x: f32,

    pub y: f32,

    pub height: Option<f32>,

    pub width: Option<f32>,

    #[wasm_bindgen(js_name = "offsetX")]
    #[serde(rename = "offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f32>,

    #[wasm_bindgen(js_name = "offsetY")]
    #[serde(rename = "offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f32>,

    #[wasm_bindgen(js_name = "advanceWidth")]
    #[serde(rename = "advanceWidth", skip_serializing_if = "Option::is_none")]
    pub advance_width: Option<f32>,
}