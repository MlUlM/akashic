use std::fmt::Debug;

use js_sys::JsString;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::console_log;
use crate::event::AkashicEvent;
use crate::event::join::JoinEvent;
use crate::game::snapshot::SnapshotSaveRequest;
use crate::game::vars::Vars;
use crate::prelude::{Scene, Trigger};
use crate::random::RandomGenerator;
use crate::resource_factory::ResourceFactory;
use crate::trigger::join::JoinHandler;
use crate::trigger::NativeTrigger;
use crate::util::FunctionIntoJsValue;

pub mod vars;
pub mod snapshot;

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
    pub fn push_scene(this: &Game, scene: Scene, options: JsValue);

    #[wasm_bindgen(method, js_name = popScene)]
    pub fn pop_scene_with_args(this: &Game, preserve: bool, step: usize);

    #[wasm_bindgen(method)]
    pub fn modified(this: &Game);

    #[wasm_bindgen(method, getter, js_name = joinedPlayerIds)]
    pub fn joined_player_ids(this: &Game) -> Box<[JsString]>;

    #[wasm_bindgen(method, js_name = raiseEvent)]
    fn _raise_event(this: &Game, event: AkashicEvent);

    #[wasm_bindgen(method, getter, js_name = onJoin)]
    fn _on_join(this: &Game) -> NativeTrigger;

    #[wasm_bindgen(method, js_name = requestSaveSnapshot)]
    fn _request_save_snapshot(this: &Game, f: JsValue);

    #[wasm_bindgen(method, js_name = requestSaveSnapshot)]
    fn _request_save_snapshot_with_owner(this: &Game, f: JsValue, owner: JsValue);
}


impl Game {
    #[inline]
    pub fn pop_scene(&self) {
        self.pop_scene_with_args(false, 1);
    }


    #[inline]
    pub fn raise_event(&self, event: impl Into<AkashicEvent>) {
        self._raise_event(event.into());
    }

    #[inline]
    pub fn request_save_snapshot<T: Serialize>(&self, f: impl FnMut() -> Option<SnapshotSaveRequest<T>> + 'static) {
        self._request(f, None);
    }


    #[inline]
    pub fn request_save_snapshot_with_owner<T: Serialize>(
        &self,
        f: impl FnMut() -> Option<SnapshotSaveRequest<T>> + 'static,
        owner: impl Into<JsValue>,
    ) {
        self._request(f, Some(owner.into()));
    }

    fn _request<T: Serialize>(
        &self,
        mut f: impl FnMut() -> Option<SnapshotSaveRequest<T>> + 'static,
        owner: Option<JsValue>,
    ) {
        let f = convert(move || {
            let j = f()
                .and_then(|snapshot| serde_wasm_bindgen::to_value(&snapshot).ok())
                .unwrap_or(JsValue::NULL);
            console_log!("{j:?}");
            j
        });

        if let Some(owner) = owner {
            self._request_save_snapshot_with_owner(f, owner);
        } else {
            self._request_save_snapshot(f);
        }
    }
}


#[inline(always)]
fn convert(f: impl FnMut() -> JsValue + 'static) -> JsValue {
    f.into_js_value()
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
