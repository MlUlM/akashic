use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::font::bitmap::glyph_area::GlyphArea;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitmapFontGlyphInfo {
    pub map: HashMap<String, GlyphArea>,
    pub height: Option<f32>,
    pub width: Option<f32>,
}

