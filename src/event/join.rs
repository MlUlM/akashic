use wasm_bindgen::prelude::wasm_bindgen;

use crate::player::Player;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type JoinEvent;

    #[wasm_bindgen(method, getter)]
    pub fn player(this: &JoinEvent) -> Player;
}



