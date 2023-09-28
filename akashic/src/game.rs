use std::fmt::Debug;
use wasm_bindgen::JsValue;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::event::AkashicEvent;

use crate::event::join::JoinEvent;
use crate::game::vars::Vars;
use crate::prelude::Trigger;
use crate::random::RandomGenerator;
use crate::resource_factory::ResourceFactory;
use crate::scene::Scene;
use crate::trigger::join::JoinHandler;
use crate::trigger::NativeTrigger;

pub mod vars;

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

    #[wasm_bindgen(method, getter)]
    pub fn random(this: &Game) -> RandomGenerator;

    #[wasm_bindgen(method, getter, js_name = localRandom)]
    pub fn local_random(this: &Game) -> RandomGenerator;

    #[wasm_bindgen(method, getter, js_name = selfId)]
    pub fn self_id(this: &Game) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn vars(this: &Game) -> Vars;

    #[wasm_bindgen(method, getter, js_name = resourceFactory)]
    pub fn resource_factory(this: &Game) -> ResourceFactory;

    #[wasm_bindgen(method, js_name = pushScene)]
    pub fn push_scene(this: &Game, scene: Scene, options: JsValue) -> NativeTrigger;

    #[wasm_bindgen(method)]
    pub fn modified(this: &Game);

    #[wasm_bindgen(method, js_name = raiseEvent)]
    fn _raise_event(this: &Game, event: AkashicEvent);

    #[wasm_bindgen(method, getter, js_name = onJoin)]
    fn _on_join(this: &Game) -> NativeTrigger;
}


impl Game {
    #[inline]
    pub fn raise_event(&self, event: impl Into<AkashicEvent>){
        self._raise_event(event.into());
    }
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
