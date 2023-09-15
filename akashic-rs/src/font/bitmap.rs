use derive_builder::Builder;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::Src;
use crate::font::bitmap::glyph_area::GlyphArea;
use crate::font::bitmap::glyph_info::BitmapFontGlyphInfo;
use crate::font::Font;

pub mod glyph_info;
pub mod glyph_area;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type BitmapFont;

    #[wasm_bindgen(constructor)]
    pub fn new(param: BitmapFontParameter) -> BitmapFont;
}


#[allow(clippy::from_over_into)]
impl Into<Font> for BitmapFont {
    #[inline]
    fn into(self) -> Font {
        self.obj.unchecked_into()
    }
}


#[non_exhaustive]
#[wasm_bindgen(getter_with_clone)]
#[derive(Builder, Debug)]
#[builder(
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct BitmapFontParameter {
    #[builder(setter(custom))]
    pub src: JsValue,

    #[wasm_bindgen(js_name = defaultGlyphHeight)]
    #[builder(setter(into, strip_option), default)]
    pub default_glyph_height: Option<f32>,

    #[wasm_bindgen(js_name = defaultGlyphWidth)]
    #[builder(setter(into, strip_option), default)]
    pub default_glyph_width: Option<f32>,

    #[builder(setter(into, strip_option), default)]
    pub map: JsValue,

    #[wasm_bindgen(js_name = glyphInfo)]
    #[builder(setter(custom, strip_option), default)]
    pub glyph_info: JsValue,

    #[wasm_bindgen(js_name = missingGlyph)]
    #[builder(setter(into, strip_option), default)]
    pub missing_glyph: Option<GlyphArea>,
}


impl BitmapFontParameterBuilder {
    pub fn new(src: Src) -> Self {
        Self {
            src: Some(src.into()),
            default_glyph_height: None,
            default_glyph_width: None,
            map: None,
            glyph_info: None,
            missing_glyph: None,
        }
    }

    #[allow(deprecated)]
    pub fn glyph_info(&mut self, glyph_info: &str) -> &mut Self {
        let glyph_info: BitmapFontGlyphInfo = serde_json::from_str(glyph_info).unwrap();
        let glyph_info = JsValue::from_serde(&glyph_info).unwrap();

        self.glyph_info = Some(glyph_info);
        self
    }

    #[inline]
    pub fn build(&self) -> BitmapFontParameter {
        self
            .fallible_build()
            .unwrap()
    }
}


