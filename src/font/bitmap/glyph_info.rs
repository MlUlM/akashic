use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::font::bitmap::glyph_area::GlyphArea;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitmapFontGlyphInfo {
    pub map: HashMap<String, GlyphArea>,

    pub height: Option<isize>,

    pub width: Option<isize>,

    #[serde(rename = "offsetX")]
    pub offset_x: Option<f32>,

    #[serde(rename = "offsetY")]
    pub offset_y: Option<f32>,

    #[serde(rename = "advanceWidth")]
    pub advance_width: Option<isize>,
}

