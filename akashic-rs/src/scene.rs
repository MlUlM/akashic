use std::fmt::{Debug};

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicScene;

use crate::entity::E;

pub mod param;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = g)]
    #[derive(Clone, Debug, AkashicScene)]
    pub type Scene;

    #[wasm_bindgen(js_namespace = g, constructor)]
    pub fn new(param: param::SceneParameterObject) -> Scene;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn local(this: &Scene) -> String;

    #[wasm_bindgen(js_namespace = g, method, getter, js_name = _loaded)]
    pub fn loaded(this: &Scene) -> bool;

    #[wasm_bindgen(js_namespace = g, method, js_name = append)]
    fn _append(this: &Scene, e: JsValue);
}


impl Scene {
    pub fn append(&self, e: &impl E) {
        self._append(e.as_js_value())
    }
}


