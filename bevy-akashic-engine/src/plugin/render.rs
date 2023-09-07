use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Entity, Event, EventReader, EventWriter, Query, RemovedComponents, ResMut, Resource};
use bevy::utils::HashMap;

use akashic_rs::prelude::EntityDestroy;
use akashic_rs::prelude::GAME;

use crate::plugin::transform::transform_system;
use crate::prelude::AkashicEntityId;

#[derive(Copy, Clone, Debug, Default, Event, Eq, PartialEq)]
pub(crate) struct SceneModifiedEvent;


pub struct AkashicRenderPlugin;

impl Plugin for AkashicRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEntityMap>()
            .add_event::<SceneModifiedEvent>()
            .add_systems(Last, (
                register_akashic_entity_system,
                akashic_entity_despawn_system,
                transform_system,
                rendering_system
            ));
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
        GAME.modified();
    }
}