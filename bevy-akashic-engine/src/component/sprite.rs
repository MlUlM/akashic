use bevy::prelude::{Bundle, Transform};

use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::prelude::{AddAkashicEntity, AkashicEntityId};
use crate::prelude::entity_size::PreviousAkashicEntitySize;

#[derive(Debug, Bundle)]
pub struct AkashicSpriteBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
    previous_size: PreviousAkashicEntitySize,
    add: AddAkashicEntity,
}


impl AsBundle<AkashicSpriteBundle> for akashic_rs::prelude::Sprite {
    fn as_bundle(&self) -> AkashicSpriteBundle {
        let size = AkashicEntitySize::new(self);
        let transform = Transform::from_xyz(self.x(), self.y(), 0.);

        AkashicSpriteBundle {
            id: AkashicEntityId(self.id()),
            transform,
            size,
            previous_size: PreviousAkashicEntitySize::from(size),
            add: AddAkashicEntity(self.clone().into()),
        }
    }
} 
