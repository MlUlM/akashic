use std::collections::HashMap;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Entity, IntoSystemConfigs, NonSend, Query, ResMut, Resource};
use crate::plugin::scheduler::GameScene;
use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::AkashicEntityId;

pub struct AkashicEntityMapPlugin;

impl Plugin for AkashicEntityMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEntityMap>()
            .add_systems(Last, (
                register_akashic_entity_system
            ).in_set(AkashicSystemSet::Added));
    }
}


#[derive(Resource, Default)]
pub(crate) struct AkashicEntityMap(pub(crate) HashMap<Entity, akashic_rs::entity::Entity>);


impl AkashicEntityMap {
    #[inline(always)]
    pub fn get(&self, entity: &Entity) -> Option<&akashic_rs::entity::Entity> {
        self.0.get(entity)
    }
}


unsafe impl Send for AkashicEntityMap {}

unsafe impl Sync for AkashicEntityMap {}


fn register_akashic_entity_system(
    mut akashic_entity_map: ResMut<AkashicEntityMap>,
    entities: Query<(Entity, &AkashicEntityId), Added<AkashicEntityId>>,
    scene: NonSend<GameScene>
) {
    for (entity, AkashicEntityId(id)) in entities.iter() {
        let Some(akashic_entity) = scene.find_child(*id) else {continue;};
        akashic_entity_map.0.insert(entity, akashic_entity.clone());
    }
}