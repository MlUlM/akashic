use bevy::prelude::{Component, Deref};

pub mod entity_size;
pub mod player_id;
pub mod entity;

pub mod prelude {
    pub use crate::component::AkashicEntityId;
    pub use crate::component::entity::filled_rect::FilledRectBundle;
}

#[derive(Component, Copy, Clone, Debug, Deref, Eq, PartialEq)]
pub struct AkashicEntityId(pub(crate) usize);


#[derive(Component, Clone, Debug)]
pub(crate) struct AddAkashicEntity(pub(crate) akashic_rs::entity::Entity);


unsafe impl Send for AddAkashicEntity{}
unsafe impl Sync for AddAkashicEntity{}

