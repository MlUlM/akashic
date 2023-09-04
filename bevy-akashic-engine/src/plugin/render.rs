use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Entity, Query, RemovedComponents, ResMut, Resource, Transform};
use bevy::utils::HashMap;

use akashic_rs::prelude::{EntityDestroy, EntitySize};
use akashic_rs::prelude::GAME;

use crate::prelude::AkashicEntityId;
use crate::prelude::entity_size::AkashicEntitySize;

pub struct AkashicRenderPlugin;

impl Plugin for AkashicRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEntityMap>()
            .add_systems(Last, (
                register_akashic_entity_system,
                transform_system,
                entity_size_system,
                akashic_entity_despawn_system
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

fn transform_system(
    transforms: Query<(&AkashicEntityId, &Transform)>
) {
    for (AkashicEntityId(id), transform) in transforms.iter() {
        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        entity.set_x(transform.translation.x);
        entity.set_y(transform.translation.y);
        entity.modified();
    }
}


fn entity_size_system(
    size_queries: Query<(&AkashicEntityId, &AkashicEntitySize)>
) {
    for (AkashicEntityId(id), size) in size_queries.iter() {
        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        entity.set_width(size.width());
        entity.set_height(size.height());
        entity.modified();
    }
}


fn akashic_entity_despawn_system(
    mut removed: RemovedComponents<AkashicEntityId>,
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
) {
    for entity in &mut removed {
        let Some(akashic_entity_id) = akashic_entity_map.0.remove(&entity) else { continue; };
        let Some(akashic_entity) = GAME.scene().find_child(*akashic_entity_id) else { continue; };
        akashic_entity.destroy();
        akashic_entity.modified();
    }
}