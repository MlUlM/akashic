use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::PointEventBase;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, PointEventBase)]
    pub type PointDownEvent;
}
