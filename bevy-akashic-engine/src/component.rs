use bevy::prelude::{Component, Deref};

mod filled_rect;
mod transform;

pub mod prelude{
    pub use crate::component::AkashicEntityId;
    pub use crate::component::filled_rect::FilledRectBundle;
    pub use crate::component::transform::AkashicTransform;
}

#[derive(Component, Copy, Clone, Debug, Deref, Eq, PartialEq)]
pub struct AkashicEntityId(pub(crate) usize);



