use std::fmt::{Debug, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Clone)]
    pub type BinaryAsset;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &BinaryAsset) -> Box<[u8]>;
}


impl Debug for BinaryAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("BinaryAsset")
            .field("data", &self.data())
            .finish()
    }
}