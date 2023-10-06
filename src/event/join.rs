use wasm_bindgen::prelude::wasm_bindgen;

use crate::player::Player;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    /// The event representing that a player has joined.
    ///
    /// ### Notes
    ///
    /// JoinEvent is occurred when the player jointed,
    /// but the case of the running on "nico-live" is a bit special.
    ///
    /// In this case, the event will only fire when the streamer joins.
    #[derive(Clone, Debug)]
    pub type JoinEvent;

    #[wasm_bindgen(method, getter)]
    pub fn player(this: &JoinEvent) -> Player;
}



