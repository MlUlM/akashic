use bevy::math::Vec2;
use bevy::prelude::{Bundle, Quat, Transform};
use bevy::utils::default;

use akashic_rs::prelude::{E, FilledRect};

use crate::command::AsBundle;
use crate::component::AkashicEntityId;
use crate::prelude::entity_size::AkashicEntitySize;
use crate::prelude::NativeAkashicEntity;


#[derive(Bundle)]
pub struct FilledRectBundle {
    id: AkashicEntityId,
    size: AkashicEntitySize,
    transform: Transform,
    native: NativeAkashicEntity
}



impl AsBundle<FilledRectBundle> for FilledRect {
    fn as_bundle(&self) -> FilledRectBundle {
        let transform = Transform {
            translation: Vec2::new(self.x(), self.y()).extend(0.),
            rotation: Quat::from_rotation_z(self.angle()),
            ..default()
        };
        
        FilledRectBundle {
            id: AkashicEntityId(self.id()),
            size: AkashicEntitySize::new(self),
            transform,
            native: NativeAkashicEntity::new(self)
        }
    }
}


