use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::Src;
use crate::prelude::Scene;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = g)]
    #[derive(Clone, Debug, crate::entity::AkashicEntity)]
    pub type Sprite;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: SpriteParameterObject) -> Sprite;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn local(this: &Sprite) -> String;
}


#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct SpriteParameterObject {
    pub scene: Scene,
    pub src: JsValue,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub local: bool,
    pub x: Option<f32>,
    pub y: Option<f32>,
}


impl SpriteParameterObject {
    #[inline(always)]
    pub fn builder(scene: Scene, src: Src) -> SpriteParameterObjectBuilder {
        SpriteParameterObjectBuilder::new(scene, src)
    }
}


pub struct SpriteParameterObjectBuilder {
    scene: Scene,
    src: Src,
    width: Option<f32>,
    height: Option<f32>,
    local: bool,
    x: Option<f32>,
    y: Option<f32>,
}


impl SpriteParameterObjectBuilder {
    pub fn new(scene: Scene, src: Src) -> Self {
        Self {
            scene,
            src,
            width: None,
            height: None,
            local: false,
            x: None,
            y: None,
        }
    }

    #[inline]
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }


    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }


    #[inline]
    pub fn x(mut self, x: f32) -> Self {
        self.x = Some(x);
        self
    }


    #[inline]
    pub fn y(mut self, y: f32) -> Self {
        self.y = Some(y);
        self
    }


    pub fn build(self) -> SpriteParameterObject {
        SpriteParameterObject {
            scene: self.scene,
            src: self.src.into(),
            width: self.width,
            height: self.height,
            local: self.local,
            x: self.x,
            y: self.y,
        }
    }
}


#[allow(clippy::from_over_into)]
impl Into<JsValue> for Src {
    fn into(self) -> JsValue {
        match self {
            Self::Surface(surface) => surface.into(),
            Self::ImageAsset(image_asset) => image_asset.into()
        }
    }
}

