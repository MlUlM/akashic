use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicEntity;

pub mod filled_rect;
pub mod sprite;
pub mod label;


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
    #[derive(Clone, AkashicEntity, Debug)]
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


pub trait AppendEntity{
    fn append(&self, child: impl Into<Entity>);
}