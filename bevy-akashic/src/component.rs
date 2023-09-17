use bevy::prelude::{Component, Deref};

pub mod player_id;
pub mod text;
pub mod object2d;
pub mod scene;

pub mod prelude {
    pub use crate::component::AkashicEntityId;
    pub use crate::component::object2d::entity::filled_rect::FilledRectBundle;
}

#[derive(Component, Copy, Clone, Debug, Deref, Eq, PartialEq)]
pub struct AkashicEntityId(pub(crate) isize);


#[derive(Component, Clone, Debug, Deref)]
pub(crate) struct NativeAkashicEntity(pub(crate) akashic_rs::object2d::entity::AkashicEntity);


unsafe impl Send for NativeAkashicEntity {}

unsafe impl Sync for NativeAkashicEntity {}
