use bevy::app::{App, Last, Plugin, PostUpdate};
use bevy::prelude::{Added, Entity, Event, EventReader, EventWriter, Query, RemovedComponents, ResMut, Resource, Transform};
use bevy::utils::HashMap;

use akashic_rs::prelude::{EntityDestroy, EntitySize};
use akashic_rs::prelude::GAME;

use crate::prelude::AkashicEntityId;
use crate::prelude::entity_size::AkashicEntitySize;

#[derive(Copy, Clone, Debug, Default, Event, Eq, PartialEq)]
pub struct RequestRenderingEvent;


pub struct AkashicRenderPlugin;

impl Plugin for AkashicRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEntityMap>()
            .add_event::<RequestRenderingEvent>()
            .add_systems(PostUpdate, (
                register_akashic_entity_system,
                transform_system,
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

fn transform_system(
    transforms: Query<(&AkashicEntityId, &Transform)>,
    mut ew: EventWriter<RequestRenderingEvent>,
) {
    for (AkashicEntityId(id), transform) in transforms.iter() {
        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        let pos = transform.translation;
        if entity.x() != pos.x || entity.y() != pos.y {
            entity.set_x(pos.x);
            entity.set_y(pos.y);
            ew.send(RequestRenderingEvent);
        }
    }
}


fn entity_size_system(
    size_queries: Query<(&AkashicEntityId, &AkashicEntitySize)>,
    mut ew: EventWriter<RequestRenderingEvent>,
) {
    for (AkashicEntityId(id), size) in size_queries.iter() {
        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        if entity.y() != size.y || entity.x() != size.x {
            entity.set_width(size.width());
            entity.set_height(size.height());
            ew.send(RequestRenderingEvent);
        }
    }
}


fn akashic_entity_despawn_system(
    mut removed: RemovedComponents<AkashicEntityId>,
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    mut ew: EventWriter<RequestRenderingEvent>,
) {
    for entity in &mut removed {
        let Some(akashic_entity_id) = akashic_entity_map.0.remove(&entity) else { continue; };
        let Some(akashic_entity) = GAME.scene().find_child(*akashic_entity_id) else { continue; };
        akashic_entity.destroy();
        ew.send(RequestRenderingEvent);
    }
}


fn rendering_system(
    er: EventReader<RequestRenderingEvent>
) {
    if !er.is_empty() {
        GAME.scene().modified();
    }
}