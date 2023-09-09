use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::player::Player;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type MessageEvent;

    #[wasm_bindgen(constructor)]
    fn _new(data: JsValue, player: Option<Player>, local: Option<bool>, event_flags: Option<u8>) -> MessageEvent;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &MessageEvent) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn event_type(this: &MessageEvent) -> String;
}


impl MessageEvent {
    #[inline]
    pub fn from_js_value(
        data: impl Into<JsValue>,
        player: Option<Player>,
        local: Option<bool>,
        event_flags: Option<u8>,
    ) -> Self {
        Self::_new(data.into(), player, local, event_flags)
    }

    #[inline]
    pub fn from_serde(
        data: &impl Serialize,
        player: Option<Player>,
        local: Option<bool>,
        event_flags: Option<u8>,
    ) -> Self {
        Self::_new(serde_wasm_bindgen::to_value(data).unwrap(), player, local, event_flags)
    }
}