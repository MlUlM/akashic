use bevy::math::Vec2;
use bevy::prelude::Bundle;
use akashic_rs::prelude::E;
use akashic_rs::filled_rect::FilledRect;
use crate::command::AsBundle;
use crate::component::AkashicEntityId;
use crate::component::transform::AkashicTransform;
#[derive(Bundle)]
pub struct FilledRectBundle {
    id: AkashicEntityId,
    transform: AkashicTransform,
}

impl AsBundle<FilledRectBundle> for FilledRect {
    fn as_bundle(&self) -> FilledRectBundle {
        FilledRectBundle {
            id: AkashicEntityId(self.id()),
            transform: AkashicTransform {
                transition: Vec2::new(self.x(), self.y()).extend(0.),
            },
        }
    }
}


