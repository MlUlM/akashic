use bevy::math::Quat;
use bevy::prelude::{Bundle, Transform};

use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::prelude::{AddAkashicEntity, AkashicEntityId};

#[derive(Debug, Bundle)]
pub struct AkashicSpriteBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
    add: AddAkashicEntity,
}


impl AsBundle<AkashicSpriteBundle> for akashic_rs::prelude::Sprite {
    fn as_bundle(&self) -> AkashicSpriteBundle {
        let size = AkashicEntitySize::new(self);
        let mut transform = Transform::from_xyz(self.x(), self.y(), 0.);
        transform.rotation = Quat::from_rotation_z(self.angle());

        AkashicSpriteBundle {
            id: AkashicEntityId(self.id()),
            transform,
            size,
            add: AddAkashicEntity(self.clone().into()),
        }
    }
} 
