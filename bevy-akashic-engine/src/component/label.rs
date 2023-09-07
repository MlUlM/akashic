use bevy::prelude::{Bundle, Transform};
use akashic_rs::entity::label::Label;

use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::component::previous_transform::PreviousTransform;
use crate::prelude::AkashicEntityId;
use crate::prelude::entity_size::PreviousAkashicEntitySize;

#[derive(Debug, Bundle)]
pub struct AkashicLabelBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
    previous_size: PreviousAkashicEntitySize,
}


impl AsBundle<AkashicLabelBundle> for Label {
    fn as_bundle(&self) -> AkashicLabelBundle {
        let size = AkashicEntitySize::new(self);
        let transform = Transform::from_xyz(self.x(), self.y(), 0.);

        AkashicLabelBundle {
            id: AkashicEntityId(self.id()),
            transform,
            size,
            previous_size: PreviousAkashicEntitySize::from(size),
        }
    }
} 
