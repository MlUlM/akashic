use bevy::math::Vec2;
use bevy::prelude::{Bundle, Transform};
use bevy::utils::default;

use akashic_rs::prelude::{E, FilledRect};


use crate::command::AsBundle;
use crate::component::AkashicEntityId;

#[derive(Bundle)]
pub struct FilledRectBundle {
    id: AkashicEntityId,
    transform: Transform,
}


impl AsBundle<FilledRectBundle> for FilledRect {
    fn as_bundle(&self) -> FilledRectBundle {
        FilledRectBundle {
            id: AkashicEntityId(self.id()),
            transform: Transform {
                translation: Vec2::new(self.x(), self.y()).extend(0.),
                ..default()
            },
        }
    }
}


