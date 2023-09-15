use bevy::app::{App, Last, Plugin};
use bevy::prelude::{NonSendMut, RemovedComponents};
use akashic_rs::entity::EntityDestroy;

use crate::plugin::append::AkashicEntityMap;
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
    mut akashic_entity_map: NonSendMut<AkashicEntityMap>,
    mut removed: RemovedComponents<AkashicEntityId>,
) {
    for entity in &mut removed {
        let Some(akashic_entity) = akashic_entity_map.remove(&entity) else { continue; };
        akashic_entity.destroy();
    }
}


