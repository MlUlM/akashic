use wasm_bindgen::JsValue;

use crate::object2d::entity::{Entity, EntityObject2D};
use crate::scene::Scene;

#[derive(Clone)]
pub enum Parent {
    Scene(Scene),
    Entity(Entity),
}

impl Parent {
    pub fn as_js_value(&self) -> JsValue {
        match self {
            Self::Scene(scene) => scene.obj(),
            Self::Entity(entity) => entity.as_js_value()
        }
    }
}




