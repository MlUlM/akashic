use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Entity, Event, EventReader, EventWriter, Query, RemovedComponents, ResMut, Resource};
use bevy::utils::HashMap;

use akashic_rs::prelude::EntityDestroy;
use akashic_rs::prelude::GAME;

use crate::plugin::transform::transform_system;
use crate::prelude::{AddAkashicEntity, AkashicEntityId};

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
pub(crate) struct AkashicEntityMap(pub(crate) HashMap<Entity, akashic_rs::entity::Entity>);


unsafe impl Send for AkashicEntityMap{}
unsafe impl Sync for AkashicEntityMap{}


fn register_akashic_entity_system(
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    entities: Query<(Entity, &AddAkashicEntity),  Added<AddAkashicEntity>>,
) {
    for (entity, AddAkashicEntity(akashic_entity)) in entities.iter() {
        akashic_entity_map.0.insert(entity, akashic_entity.clone());
    }
}


fn akashic_entity_despawn_system(
    mut removed: RemovedComponents<AkashicEntityId>,
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    mut ew: EventWriter<SceneModifiedEvent>,
) {
    for entity in &mut removed {
        let Some(akashic_entity) = akashic_entity_map.0.remove(&entity) else { continue; };

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