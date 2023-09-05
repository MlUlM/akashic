use std::fmt::Debug;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::scene::Scene;
use crate::trigger::NativeTrigger;


pub mod prelude{
    pub use crate::game::{GAME, Game};
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_namespace = g)]
    pub type Game;

    #[wasm_bindgen(js_namespace = g, js_name = "game")]
    pub static GAME: Game;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn age(this: &Game) -> f32;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn fps(this: &Game) -> f32;

    #[wasm_bindgen(js_namespace = g, method)]
    pub fn scene(this: &Game) -> Scene;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn width(this: &Game) -> f32;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn height(this: &Game) -> f32;

    #[wasm_bindgen(js_namespace = g, method, js_name = pushScene)]
    pub fn push_scene(this: &Game, scene: Scene, options: JsValue) -> NativeTrigger;
}




impl Default for Game {
    fn default() -> Self {
        GAME.clone()
    }
}






