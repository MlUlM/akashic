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


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptionNumber(Option<f32>);


impl From<f32> for OptionNumber {
    fn from(value: f32) -> Self {
        Self(Some(value))
    }
}

impl Default for OptionNumber {
    fn default() -> Self {
        Self(None)
    }
}