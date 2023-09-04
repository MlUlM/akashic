use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicEntity;

mod filled_rect;
mod sprite;


pub mod prelude {
    pub use crate::entity::{
        E,
        Entity,
        EntitySize,
        sprite::*,
        filled_rect::*,
        EntityDestroy
    };
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, AkashicEntity)]
    pub type Entity;
}


#[auto_delegate::delegate]
pub trait E {
    fn id(&self) -> usize;

    fn as_js_value(&self) -> JsValue;
}


pub trait EntitySize {
    fn width(&self) -> f32;

    fn set_width(&self, width: f32);

    fn height(&self) -> f32;

    fn set_height(&self, height: f32);
}


pub trait EntityDestroy{
    fn destroy(&self);


    fn destroy_with_surface(&self);
}