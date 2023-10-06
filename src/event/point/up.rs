use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::{PointDeltaEventBase};

#[wasm_bindgen]
extern "C" {
    /// This event occurs when pointer is upped move after [`PointerDownEvent`](crate::event::point::down::PointDownEvent) occurred.
    #[derive(Clone, PointDeltaEventBase)]
    pub type PointUpEvent;
}