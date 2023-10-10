use std::fmt::Debug;
use derive_builder::Builder;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::font::Font;
use crate::font::font_family::FontFamily;
use crate::font::font_weight_string::FontWeightString;
use crate::font::surface_atlas_set::SurfaceAtlasSet;
use crate::game::Game;
use crate::prelude::OptionNumber;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type DynamicFont;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: DynamicFontParam) -> DynamicFont;
}


impl Default for DynamicFont {
    fn default() -> Self {
        DynamicFontBuilder::new(FontFamily::sans_serif(), 32.)
            .build()
    }
}

#[allow(clippy::from_over_into)]
impl Into<Font> for DynamicFont {
    #[inline]
    fn into(self) -> Font {
        self.unchecked_into()
    }
}


#[non_exhaustive]
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Builder)]
#[builder(
name = "DynamicFontBuilder",
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct DynamicFontParam {
    #[builder(default)]
    pub game: Game,

    #[wasm_bindgen(js_name = fontFamily)]
    pub font_family: FontFamily,

    pub size: f32,

    #[builder(setter(into, strip_option), default)]
    pub hint: Option<DynamicFontHint>,

    #[wasm_bindgen(js_name = fontColor)]
    #[builder(setter(into, strip_option), default)]
    pub font_color: Option<String>,

    #[wasm_bindgen(js_name = fontWeight)]
    #[builder(setter(into, strip_option), default)]
    pub font_weight: Option<FontWeightString>,

    #[wasm_bindgen(js_name = strokeWidth)]
    #[builder(setter(into, strip_option), default)]
    pub stroke_width: Option<f32>,

    #[builder(setter(into, strip_option), default)]
    #[wasm_bindgen(js_name = strokeColor)]
    pub stroke_color: Option<String>,

    #[builder(setter(into, strip_option), default)]
    #[wasm_bindgen(js_name = strokeOnly)]
    pub stroke_only: Option<bool>,

    #[builder(setter(into, strip_option), default)]
    #[wasm_bindgen(js_name = surfaceAtlasSet)]
    pub surface_atlas_set: Option<SurfaceAtlasSet>,
}


impl DynamicFontBuilder {
    #[inline]
    pub fn new(
        font_family: FontFamily,
        size: f32,
    ) -> Self {
        Self {
            font_family: Some(font_family),
            size: Some(size),
            ..DynamicFontBuilder::empty()
        }
    }


    #[inline]
    pub fn build(&self) -> DynamicFont {
        DynamicFont::new(self.fallible_build().unwrap())
    }
}


#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct DynamicFontHint {
    #[wasm_bindgen(js_name = initialAtlasWidth)]
    pub initial_atlas_width: OptionNumber,

    #[wasm_bindgen(js_name = initialAtlasHeight)]
    pub initial_atlas_height: OptionNumber,

    #[wasm_bindgen(js_name = maxAtlasWidth)]
    pub max_atlas_width: OptionNumber,

    #[wasm_bindgen(js_name = maxAtlasHeight)]
    pub max_atlas_height: OptionNumber,

    #[wasm_bindgen(js_name = maxAtlasNum)]
    pub max_atlas_num: OptionNumber,
}