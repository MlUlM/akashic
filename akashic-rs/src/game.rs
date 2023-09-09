use std::fmt::Debug;
use wasm_bindgen::JsValue;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::event::message::MessageEvent;
use crate::prelude::Scene;
use crate::trigger::NativeTrigger;

pub mod prelude {
    pub use crate::game::{GAME, Game};
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type Game;

    #[wasm_bindgen(js_name = "game")]
    pub static GAME: Game;

    #[wasm_bindgen(getter, method)]
    pub fn age(this: &Game) -> f32;

    #[wasm_bindgen(getter, method)]
    pub fn fps(this: &Game) -> f32;

    #[wasm_bindgen(method)]
    pub fn scene(this: &Game) -> Scene;

    #[wasm_bindgen(getter, method)]
    pub fn width(this: &Game) -> f32;

    #[wasm_bindgen(getter, method)]
    pub fn height(this: &Game) -> f32;

    #[wasm_bindgen(method, js_name = pushScene)]
    pub fn push_scene(this: &Game, scene: Scene, options: JsValue) -> NativeTrigger;

    #[wasm_bindgen(method)]
    pub fn modified(this: &Game);

    #[wasm_bindgen(method, js_name= raiseEvent)]
    pub fn raise_event(this: &Game, event: MessageEvent);
}


impl Default for Game {
    fn default() -> Self {
        GAME.clone()
    }
}






