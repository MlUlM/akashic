use std::fmt::Debug;
use wasm_bindgen::JsValue;

use crate::event::join::JoinEvent;
use crate::event::message::MessageEvent;
use crate::prelude::{Scene, Trigger};
use crate::trigger::join::JoinHandler;
use crate::trigger::NativeTrigger;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod prelude {
    pub use crate::game::{Game, GAME};
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

    #[wasm_bindgen(method, getter, js_name = selfId)]
    pub fn self_id(this: &Game) -> Option<String>;

    #[wasm_bindgen(method, js_name = pushScene)]
    pub fn push_scene(this: &Game, scene: Scene, options: JsValue) -> NativeTrigger;

    #[wasm_bindgen(method)]
    pub fn modified(this: &Game);

    #[wasm_bindgen(method, js_name = raiseEvent)]
    pub fn raise_event(this: &Game, event: MessageEvent);

    #[wasm_bindgen(method, getter, js_name = onJoin)]
    fn _on_join(this: &Game) -> NativeTrigger;
}

impl JoinHandler for Game {
    #[inline(always)]
    fn on_join(&self) -> Trigger<JoinEvent> {
        Trigger::new(self._on_join())
    }
}

impl Default for Game {
    fn default() -> Self {
        GAME.clone()
    }
}
