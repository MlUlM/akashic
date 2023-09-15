use std::collections::HashMap;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Deref, DerefMut, Entity, IntoSystemConfigs, NonSend, NonSendMut, Parent, Query};

use crate::plugin::scheduler::GameScene;

use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::NativeAkashicEntity;

pub struct AkashicAppendEntityPlugin;


impl Plugin for AkashicAppendEntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_non_send_resource::<AkashicEntityMap>()
            .add_systems(Last, (
                append_akashic_entities_system
            ).in_set(AkashicSystemSet::Added));
    }
}


fn append_akashic_entities_system(
    mut entity_map: NonSendMut<AkashicEntityMap>,
    akashic_entities: Query<(Entity, &NativeAkashicEntity, Option<&Parent>), Added<NativeAkashicEntity>>,
    scene: NonSend<GameScene>,
) {
    for (entity, native, parent) in akashic_entities.iter() {
        entity_map.insert(entity, native.0.clone());

        if let Some(parent) = parent {} else {
            scene.append(&native.0);
        }
    }
}


#[derive(Default, Deref, DerefMut)]
pub(crate) struct AkashicEntityMap(pub(crate) HashMap<Entity, akashic_rs::entity::Entity>);

