use std::hash::Hash;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::Query;

use akashic_rs::console_log;
use akashic_rs::entity::E;
use akashic_rs::game::GAME;

use crate::prelude::{AkashicEntityId, AkashicTransform};

pub struct AkashicRenderPlugin;

impl Plugin for AkashicRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, transform_system);
    }
}


fn transform_system(
    transforms: Query<(&AkashicEntityId, &AkashicTransform)>
) {
    for (AkashicEntityId(id), transform) in transforms.iter() {
        let scene = GAME.scene().children();
        let Some(entity) = scene.iter().find(|e| &e.id() == id) else { continue; };
        entity.set_x(transform.transition.x);
        entity.set_y(transform.transition.y);
        entity.modified();
    }
}