use bevy::prelude::{Bundle, Transform};

use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::prelude::AkashicEntityId;

#[derive(Debug, Bundle)]
pub struct AkashicSpriteBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
}


impl AsBundle<AkashicSpriteBundle> for akashic_rs::prelude::Sprite {
    fn as_bundle(&self) -> AkashicSpriteBundle {
        AkashicSpriteBundle {
            id: AkashicEntityId(self.id()),
            transform: Transform::from_xyz(self.x(), self.y(), 0.),
            size: AkashicEntitySize::new(self),
        }
    }
} 
