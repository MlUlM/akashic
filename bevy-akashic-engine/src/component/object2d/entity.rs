use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_rs::prelude::EntityObject2D;

pub mod filled_rect;
pub mod sprite;
pub mod label;
pub(crate) mod entity_bundle;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[wasm_bindgen(js_name=getEntityProperties)]
    fn _entity_properties(entity: &JsValue) -> JsValue;
}

#[inline]
pub(crate) fn entity_properties(entity: &impl EntityObject2D) -> EntityProperties{
    let raw = _entity_properties(entity.js_value_ref());
    serde_wasm_bindgen::from_value(raw).unwrap()
}


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub(crate) struct EntityProperties {
    pub id: isize,

    pub x: f32,

    pub y: f32,

    pub width: f32,

    pub height: f32,
    
    
    pub angle: f32,

    #[serde(rename = "scaleX")]
    pub scale_x: f32,

    #[serde(rename = "scaleY")]
    pub scale_y: f32,

    #[serde(rename = "anchorX")]
    pub anchor_x: Option<f32>,

    #[serde(rename = "anchorY")]
    pub anchor_y: Option<f32>,
}