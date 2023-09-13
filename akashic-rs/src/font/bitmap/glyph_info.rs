use std::collections::HashMap;
use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::font::bitmap::glyph_area::GlyphArea;

// #[wasm_bindgen(js_namespace = g)]
// extern {
//     #[derive(Clone, Debug)]
//     pub type BitmapFontGlyphInfo;
// }



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitmapFontGlyphInfo{
    pub map: HashMap<String, GlyphArea>,
    pub height: Option<f32>,
    pub width: Option<f32>,
}

