use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicEntity;
use crate::prelude::{PointDownHandler, UpdateHandler};

#[wasm_bindgen]
extern "C"{
    #[derive(Clone, AkashicEntity)]
    pub type Entity;
}



#[auto_delegate::delegate]
pub trait E: PointDownHandler + UpdateHandler {
    fn id(&self) -> usize;

    fn as_js_value(&self) -> JsValue;
}


