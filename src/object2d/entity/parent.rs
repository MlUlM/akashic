use wasm_bindgen::JsValue;

use crate::object2d::entity::{AkashicEntity, EntityObject2D};
use crate::scene::Scene;

#[derive(Clone)]
pub enum Parent {
    Scene(Scene),
    Entity(AkashicEntity),
}

impl Parent {
    pub fn as_js_value(&self) -> JsValue {
        match self {
            Self::Scene(scene) => scene.obj(),
            Self::Entity(entity) => entity.as_js_value()
        }
    }
}




