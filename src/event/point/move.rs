use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::PointDeltaEventBase;

#[wasm_bindgen]
extern "C" {
    /// This event occurs when pointer is moved move after [`PointerDownEvent`](crate::event::point::down::PointDownEvent) occurred.
    ///
    /// So, it's more like a drag event than an event like `cursor-move`
    #[derive(Clone, PointDeltaEventBase)]
    pub type PointMoveEvent;
}



