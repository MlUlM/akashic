use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::PointEventBase;

use crate::event::AkashicEvent;
use crate::prelude::{AkashicEntity, CommonOffset};

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    /// This event is occurred once when `pointerdown` occurs.
    ///
    /// Then, if the pointer moves while down, [`PointMoveEvent`](crate::event::point::mov::PointMoveEvent) will occur in succession.
    #[derive(Clone, PointEventBase)]
    pub type PointDownEvent;

    #[wasm_bindgen(constructor)]
    pub fn new(pointer_id: i32, target: Option<AkashicEntity>, point: CommonOffset) -> PointDownEvent;
}


#[allow(clippy::from_over_into)]
impl Into<AkashicEvent> for PointDownEvent {
    #[inline]
    fn into(self) -> AkashicEvent {
        self.unchecked_into()
    }
}