use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::entity::E;
use crate::scene::Scene;

#[derive(Clone)]
pub enum Parent<T: E> {
    Scene(Scene),
    Entity(T),
}

impl<T: E> Parent<T> {
    pub fn as_js_value(&self) -> JsValue {
        match self {
            Self::Scene(scene) => scene.obj(),
            Self::Entity(entity) => entity.as_js_value()
        }
    }
}



pub type  OptionNumber = Option<f32>;

