use bevy::prelude::{Component, Deref};

pub mod filled_rect;
pub mod sprite;
pub mod entity_size;
pub(crate) mod previous_transform;
pub mod label;

pub mod prelude {
    pub use crate::component::AkashicEntityId;
    pub use crate::component::filled_rect::FilledRectBundle;
}

#[derive(Component, Copy, Clone, Debug, Deref, Eq, PartialEq)]
pub struct AkashicEntityId(pub(crate) usize);



