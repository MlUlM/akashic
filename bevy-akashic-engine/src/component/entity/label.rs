use bevy::prelude::{Bundle, Transform};

use akashic_rs::entity::label::Label;
use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::prelude::AkashicEntityId;

#[derive(Debug, Bundle)]
pub struct AkashicLabelBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
}


impl AsBundle<AkashicLabelBundle> for Label {
    fn as_bundle(&self) -> AkashicLabelBundle {
        let size = AkashicEntitySize::new(self);
        let transform = Transform::from_xyz(self.x(), self.y(), 0.);

        AkashicLabelBundle {
            id: AkashicEntityId(self.id()),
            transform,
            size,
        }
    }
} 
