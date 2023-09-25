use wasm_bindgen::prelude::wasm_bindgen;

pub mod message;
pub mod join;
pub mod point;


#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_name=Event)]
    pub type AkashicEvent;
}