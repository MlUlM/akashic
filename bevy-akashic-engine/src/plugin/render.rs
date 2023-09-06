use bevy::app::{App, Last, Plugin, PostUpdate};
use bevy::prelude::{Added, Entity, Event, EventReader, EventWriter, Query, RemovedComponents, ResMut, Resource};
use bevy::utils::HashMap;

use akashic_rs::prelude::{EntityDestroy, EntitySize};
use akashic_rs::prelude::GAME;

use crate::prelude::AkashicEntityId;
use crate::prelude::entity_size::{AkashicEntitySize, PreviousAkashicEntitySize};

#[derive(Copy, Clone, Debug, Default, Event, Eq, PartialEq)]
pub(crate) struct SceneModifiedEvent;


pub struct AkashicRenderPlugin;

impl Plugin for AkashicRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEntityMap>()
            .add_event::<SceneModifiedEvent>()
            .add_systems(PostUpdate, (
                register_akashic_entity_system,
                entity_size_system,
                akashic_entity_despawn_system
            ))
            .add_systems(Last, rendering_system);
    }
}

#[derive(Resource, Default)]
struct AkashicEntityMap(HashMap<bevy::prelude::Entity, AkashicEntityId>);

fn register_akashic_entity_system(
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    entities: Query<(Entity, &AkashicEntityId), Added<AkashicEntityId>>,
) {
    for (entity, id) in entities.iter() {
        akashic_entity_map.0.insert(entity, *id);
    }
}


fn entity_size_system(
    mut size_queries: Query<(&AkashicEntityId, &AkashicEntitySize, &mut PreviousAkashicEntitySize)>,
    mut ew: EventWriter<SceneModifiedEvent>,
) {
    for (AkashicEntityId(id), size, mut previous) in size_queries.iter_mut() {
        if previous.eq(size) {
            continue;
        }

        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        if previous.x != size.x {
            entity.set_width(size.width());
        }
        if previous.y != size.y {
            entity.set_height(size.height());
        }

        *previous = PreviousAkashicEntitySize(*size);
        ew.send(SceneModifiedEvent);
    }
}


fn akashic_entity_despawn_system(
    mut removed: RemovedComponents<AkashicEntityId>,
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    mut ew: EventWriter<SceneModifiedEvent>,
) {
    for entity in &mut removed {
        let Some(akashic_entity_id) = akashic_entity_map.0.remove(&entity) else { continue; };
        let Some(akashic_entity) = GAME.scene().find_child(*akashic_entity_id) else { continue; };
        akashic_entity.destroy();
        ew.send(SceneModifiedEvent);
    }
}


fn rendering_system(
    er: EventReader<SceneModifiedEvent>
) {
    if !er.is_empty() {
        GAME.scene().modified();
    }
}