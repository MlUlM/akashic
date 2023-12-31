use std::fmt::Debug;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::Scene;

use crate::game::GAME;
use crate::object2d::entity::{AkashicEntity, EntityObject2D};
use crate::prelude::LocalTickMode;

pub mod param;


pub mod prelude {
    pub use crate::scene::{
        param::*,
        Scene,
    };
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug, Scene)]
    pub type Scene;

    #[wasm_bindgen(constructor)]
    pub fn new(param: param::SceneParam) -> Scene;

    #[wasm_bindgen(method, getter)]
    pub fn local(this: &Scene) -> LocalTickMode;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Scene) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = loaded)]
    pub fn loaded(this: &Scene) -> bool;

    #[wasm_bindgen(method)]
    pub fn unregister(this: &Scene, entity: JsValue);

    #[wasm_bindgen(method, js_name = append)]
    fn _append(this: &Scene, e: JsValue);
}


impl Scene {
    #[inline(always)]
    pub fn append(&self, e: &impl EntityObject2D) {
        self._append(e.as_js_value())
    }


    #[inline(always)]
    pub fn find_child(&self, id: isize) -> Option<AkashicEntity> {
        self
            .children()
            .iter()
            .cloned()
            .find(|child| {
                child.id() == id
            })
    }


    #[inline(always)]
    pub(crate) fn obj(&self) -> JsValue {
        self.obj.clone()
    }
}


impl Default for Scene {
    #[inline(always)]
    fn default() -> Self {
        GAME.scene()
    }
}
