use bevy::prelude::Bundle;

use crate::command::IntoBundle;
use crate::component::object2d::entity::entity_bundle::AkashicEntityBundle;

#[derive(Debug, Bundle)]
pub struct AkashicSpriteBundle {
    basic: AkashicEntityBundle,
}


impl IntoBundle<AkashicSpriteBundle> for akashic_rs::prelude::Sprite {
    fn into_bundle(self) -> AkashicSpriteBundle {
        AkashicSpriteBundle {
            basic: AkashicEntityBundle::new(self)
        }
    }
} 



