use std::fmt::Debug;
use js_sys::JsString;

use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::event::AkashicEvent;
use crate::event::join::JoinEvent;
use crate::game::snapshot::SnapshotSaveRequest;
use crate::game::vars::Vars;
use crate::prelude::Trigger;
use crate::random::RandomGenerator;
use crate::resource_factory::ResourceFactory;
use crate::scene::Scene;
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


    /// Returns a number of times g.game.tick was called.
    #[wasm_bindgen(getter, method)]
    pub fn age(this: &Game) -> f32;



    /// Returns the fps of this game.
    ///
    /// This value is automatically set based on the game.json settings.
    #[wasm_bindgen(getter, method)]
    pub fn fps(this: &Game) -> f32;


    /// Returns the current scene.
    #[wasm_bindgen(method)]
    pub fn scene(this: &Game) -> Scene;


    /// Returns the game screen width.
    ///
    /// This value is equal to the canvas size used by akasic engine.
    #[wasm_bindgen(getter, method)]
    pub fn width(this: &Game) -> f32;


    /// Returns the game screen height.
    ///
    /// This value is equal to the canvas size used by akasic engine.
    #[wasm_bindgen(getter, method)]
    pub fn height(this: &Game) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn random(this: &Game) -> RandomGenerator;

    #[wasm_bindgen(method, getter, js_name = localRandom)]
    pub fn local_random(this: &Game) -> RandomGenerator;


    /// Returns the own player-id.
    ///
    /// This method return `None` if running on the node.js.
    #[wasm_bindgen(method, getter, js_name = selfId)]
    pub fn self_id(this: &Game) -> Option<String>;


    ///　Returns the [`Vars`](Vars).
    #[wasm_bindgen(method, getter)]
    pub fn vars(this: &Game) -> Vars;

    #[wasm_bindgen(method, getter, js_name = resourceFactory)]
    pub fn resource_factory(this: &Game) -> ResourceFactory;


    /// Request to push scene to the scene stack and trasitioning to that scene.
    #[wasm_bindgen(method, js_name = pushScene)]
    pub fn push_scene(this: &Game, scene: Scene);


    /// Request to pop scenes from the scene statck.
    ///
    ///
    /// * `preserve` - if preserve is true, do not destroy the scene.
    /// * `step` - num of scenes to pop.
    #[wasm_bindgen(method, js_name = popScene)]
    pub fn pop_scene_with_args(this: &Game, preserve: bool, step: usize);

    #[wasm_bindgen(method)]
    pub fn modified(this: &Game);



    /// Returns the list of playerid that have joined the game.
    #[wasm_bindgen(method, getter, js_name = joinedPlayerIds)]
    pub fn joined_player_ids(this: &Game) -> Box<[JsString]>;

    #[wasm_bindgen(method)]
    pub fn unregister(this: &Game, entity: JsValue);

    #[wasm_bindgen(method, js_name = raiseEvent)]
    fn _raise_event(this: &Game, event: AkashicEvent);

    #[wasm_bindgen(method, js_name = replaceScene)]
    fn _replace_scene(this: &Game, scene: Scene, preserve_urrent: bool);

    #[wasm_bindgen(method, getter, js_name = onJoin)]
    fn _on_join(this: &Game) -> NativeTrigger;

    #[wasm_bindgen(method, js_name = requestSaveSnapshot)]
    fn _request_save_snapshot(this: &Game, f: JsValue);

    #[wasm_bindgen(method, js_name = requestSaveSnapshot)]
    fn _request_save_snapshot_with_owner(this: &Game, f: JsValue, owner: JsValue);
}


impl Game {
    /// Request to pop current scene from the scene statck.
    ///
    /// The poped scene will be discarded.
    #[inline]
    pub fn pop_scene(&self) {
        self.pop_scene_with_args(false, 1);
    }


    /// Request to replace the current scene with the scene passed as an argument.
    ///
    ///
    /// Replaced scene will destroyed.
    #[inline]
    pub fn replace_scene(&self, scene: Scene) {
        self._replace_scene(scene, false);
    }


    /// Request to replace the current scene with the scene passed as an argument.
    ///
    ///
    /// Replaced scene won't destroyed, so you must explicitly destory.
    #[inline]
    pub fn replace_scene_with_preserve_current(&self, scene: Scene) {
        self._replace_scene(scene, true);
    }


    /// Fires any event.
    ///
    ///
    /// The occurred events can be received using [`MessageHandler::on_message`](crate::trigger::message::MessageHandler::on_message).
    ///
    ///
    /// All players will receive this event, so you must use this method in local logics.
    #[inline]
    pub fn raise_event(&self, event: impl Into<AkashicEvent>) {
        self._raise_event(event.into());
    }


    /// Request to save snapshot.
    ///
    ///
    /// Passed callback is called when finishing this frame.
    /// if the return value of passed callbakc is `Some`, save snapshot using it.
    #[inline]
    pub fn request_save_snapshot<T: Serialize>(&self, f: impl FnMut() -> Option<SnapshotSaveRequest<T>> + 'static) {
        self._request(f, None);
    }


     /// Request to save snapshot.
     ///
     /// Basically the same as [`Game::request_save_snapshot`](Game::request_save_snapshot),
     /// but except that callback-function's `this` target to be `owner`
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
            f()
                .and_then(|snapshot| serde_wasm_bindgen::to_value(&snapshot).ok())
                .unwrap_or(JsValue::NULL)
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
