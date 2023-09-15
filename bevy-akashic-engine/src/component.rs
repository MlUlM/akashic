use bevy::prelude::{Component, Deref};

pub mod entity_size;
pub mod player_id;
pub mod entity;
pub mod text;

pub mod prelude {
    pub use crate::component::AkashicEntityId;
    pub use crate::component::entity::filled_rect::FilledRectBundle;
}

#[derive(Component, Copy, Clone, Debug, Deref, Eq, PartialEq)]
pub struct AkashicEntityId(pub(crate) usize);

#[derive(Component, Clone, Debug, Deref)]
pub(crate) struct NativeAkashicEntity(pub(crate) akashic_rs::entity::Entity);

impl NativeAkashicEntity{
    #[inline]
    pub fn new<E: Into<akashic_rs::entity::Entity> + Clone>(native: &E) -> Self{
        Self(native.clone().into())
    }
}


unsafe impl Send for NativeAkashicEntity {}

unsafe impl Sync for NativeAkashicEntity{}

// #[wasm_bindgen]
// pub(crate) struct EntityProperties {
//     pub x: f32,
//     pub y: f32,
//     pub width: f32,
//     pub height: f32,
//     pub angle: f32,
//     pub id: isize,
// }
// 
// #[macro_use]
// pub(crate) mod property {
//     macro_rules! get_properties {
//     ($entity_name: ident) => {
//             #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = g)]
//             extern {
//                 #[wasm_bindgen::prelude::wasm_bindgen(method)]
//                 pub(crate) fn get_properties(this: &$entity_name) -> crate::component::EntityProperties;
//             }
//         };
//     }
// 
//     pub(crate) use get_properties;
// }
// 
// 
