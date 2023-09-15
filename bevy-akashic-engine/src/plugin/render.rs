use bevy::app::{App, Last, Plugin};
use bevy::prelude::{RemovedComponents, ResMut};

use akashic_rs::prelude::EntityDestroy;

use crate::plugin::akashic_entity_map::AkashicEntityMap;
use crate::prelude::AkashicEntityId;

pub struct AkashicDespawnPlugin;

impl Plugin for AkashicDespawnPlugin {
    fn build(&self, despawn: &mut App) {
        despawn
            .add_systems(Last, (
                akashic_entity_despawn_system,
            ));
    }
}


fn akashic_entity_despawn_system(
    mut removed: RemovedComponents<AkashicEntityId>,
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
) {
    for entity in &mut removed {
        let Some(akashic_entity) = akashic_entity_map.0.remove(&entity) else { continue; };

        akashic_entity.destroy();
        akashic_entity.destroy();
    }
}


