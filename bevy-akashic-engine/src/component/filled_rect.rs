use bevy::math::Vec2;
use bevy::prelude::{Bundle, Transform};
use bevy::utils::default;

use akashic_rs::prelude::{E, FilledRect};

use crate::command::AsBundle;
use crate::component::AkashicEntityId;
use crate::prelude::previous_transform::PreviousTransform;

#[derive(Bundle)]
pub struct FilledRectBundle {
    id: AkashicEntityId,
    transform: Transform,
}


impl AsBundle<FilledRectBundle> for FilledRect {
    fn as_bundle(&self) -> FilledRectBundle {
        let transform = Transform {
            translation: Vec2::new(self.x(), self.y()).extend(0.),
            ..default()
        };
        
        FilledRectBundle {
            id: AkashicEntityId(self.id()),
            transform,
        }
    }
}


