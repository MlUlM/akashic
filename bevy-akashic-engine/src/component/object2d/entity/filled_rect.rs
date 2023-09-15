use bevy::prelude::Bundle;

use akashic_rs::prelude::FilledRect;

use crate::command::IntoBundle;
use crate::prelude::object2d::entity::entity_bundle::AkashicEntityBundle;

#[derive(Bundle)]
pub struct FilledRectBundle {
    basic: AkashicEntityBundle,
}


impl IntoBundle<FilledRectBundle> for FilledRect {
    fn into_bundle(self) -> FilledRectBundle {
        FilledRectBundle {
            basic: AkashicEntityBundle::new(self)
        }
    }
}


