use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{CacheableEntity, EParamSetters, object_e_parameter};

use crate::error::AkashicError;
use crate::font::Font;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug, CacheableEntity)]
    #[wasm_bindgen(js_name = Label)]
    pub type Label;

    #[wasm_bindgen(constructor)]
    pub fn new(param: LabelParam) -> Label;

    #[wasm_bindgen(method, getter)]
    pub fn text(this: &Label) -> String;

    #[wasm_bindgen(method, getter, js_name = textColor)]
    fn _text_color(this: &Label) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = textAlign)]
    fn _text_align(this: &Label) -> JsValue;
}


impl Label {
    #[inline]
    pub fn text_color(&self) -> Option<TextColor> {
        self._text_color().map(TextColor::from)
    }

    #[inline]
    pub fn text_align(&self) -> TextAlignment {
        let inner = self._text_align();
        if inner.is_undefined() {
            panic!("g.Label.textColor cannot be undefined");
        }

        if let Some(raw) = inner.as_string() {
            return TextAlignment::try_from(raw).unwrap();
        }

        // TextAlignmentというEnum型の場合、インデックス番号に変換される
        let Some(raw) = inner.as_f64() else {
            panic!("g.Label.textAlign type must be TextAlignString or TextAlign")
        };

        match raw.round() as i64 {
            0 => TextAlignment::Left,
            1 => TextAlignment::Center,
            2 => TextAlignment::Right,
            _ => panic!("unreachable")
        }
    }
}

#[non_exhaustive]
#[object_e_parameter]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Builder, EParamSetters)]
#[builder(
name = "LabelBuilder",
custom_constructor,
create_empty = "empty",
build_fn(private, name = "fallible_build")
)]
pub struct LabelParam {
    #[builder(setter(into))]
    pub text: String,

    #[builder(setter(into))]
    pub font: Font,

    #[wasm_bindgen(js_name = fontSize)]
    #[builder(setter(into, strip_option), default)]
    pub font_size: crate::option_number::OptionNumber,

    #[wasm_bindgen(js_name = textAlign)]
    #[builder(setter(custom), default)]
    pub text_align: Option<String>,

    #[wasm_bindgen(js_name = maxWidth)]
    #[builder(setter(into, strip_option), default)]
    pub max_width: crate::option_number::OptionNumber,

    #[wasm_bindgen(js_name = widthAutoAdjust)]
    #[builder(setter(into, strip_option), default)]
    pub width_auto_adjust: Option<bool>,

    #[wasm_bindgen(js_name = textColor)]
    #[builder(setter(into, strip_option), default)]
    pub text_color: Option<String>,
}


impl LabelBuilder {
    #[inline]
    pub fn new(
        text: impl Into<String>,
        font: impl Into<Font>,
    ) -> Self {
        Self {
            text: Some(text.into()),
            font: Some(font.into()),
            ..LabelBuilder::empty()
        }
    }
    
    pub fn text_alignment(&mut self, text_alignment: TextAlignment) -> &mut Self {
        self.text_align = Some(Some(text_alignment.into()));
        self
    }

    #[inline]
    pub fn build(&self) -> Label {
        Label::new(self.fallible_build().unwrap())
    }
}


#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}


#[allow(clippy::from_over_into)]
impl Into<String> for TextAlignment {
    fn into(self) -> String {
        match self {
            TextAlignment::Left => "left".to_string(),
            TextAlignment::Center => "center".to_string(),
            TextAlignment::Right => "right".to_string()
        }
    }
}


impl TryFrom<String> for TextAlignment {
    type Error = AkashicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "left" => Ok(TextAlignment::Left),
            "center" => Ok(TextAlignment::Center),
            "right" => Ok(TextAlignment::Right),
            _ => Err(AkashicError::IllegalTextAlignmentString(value))
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextColor(String);


impl TextColor {
    #[inline]
    pub fn from_rgba(
        r: u8,
        g: u8,
        b: u8,
        a: f32,
    ) -> Self {
        Self(format!("rgba({r},{g},{b},{a})"))
    }
}


impl From<String> for TextColor {
    #[inline]
    fn from(value: String) -> Self {
        TextColor(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for TextColor {
    #[inline]
    fn into(self) -> String {
        self.0
    }
}