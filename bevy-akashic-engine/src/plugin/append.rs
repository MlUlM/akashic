use std::collections::HashMap;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Added, Children, Deref, DerefMut, Entity, IntoSystemConfigs, NonSend, NonSendMut, Parent, Query};
use akashic_rs::prelude::EntityObject2D;

use crate::plugin::scene::NativeScene;
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
    parents: Query<&NativeAkashicEntity, &Children>,
    akashic_entities: Query<(Entity, &NativeAkashicEntity, Option<&Parent>), Added<NativeAkashicEntity>>,
    scene: NonSend<NativeScene>,
) {
    for (entity, native, parent) in akashic_entities.iter() {
        entity_map.insert(entity, native.0.clone());

        if let Some(parent_entity) = parent {
            // Errorの場合既に親が削除されてしまっていると考えてスキップ
            let Ok(parent) = parents.get(parent_entity.get()) else { continue; };
            parent.append(native.0.clone());
        } else {
            scene.append(&native.0);
        }
    }
}


#[derive(Default, Deref, DerefMut)]
pub(crate) struct AkashicEntityMap(pub(crate) HashMap<Entity, akashic_rs::object2d::entity::Entity>);

