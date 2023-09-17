use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Commands, Component, Entity, IntoSystemConfigs, Query, With, Without};
use akashic_rs::object2d::entity::cacheable::CacheableEntityObject2D;
use akashic_rs::prelude::EntityObject2D;


use crate::plugin::system_set::AkashicSystemSet;
use crate::component::object2d::entity::label::NativeAkashicLabel;
use crate::prelude::NativeAkashicEntity;

#[derive(Component)]
pub(crate) struct RequestModifyTarget;

#[derive(Component)]
pub(crate) struct RequestInvalidateTarget;

pub struct AkashicModifyPlugin;


impl Plugin for AkashicModifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (
                modify_system,
                invalidate_entity
            ).in_set(AkashicSystemSet::Modify));
    }
}


fn modify_system(
    mut commands: Commands,
    akashic_entities: Query<(Entity, &NativeAkashicEntity), (With<RequestModifyTarget>, Without<RequestInvalidateTarget>)>,
) {
    for (entity, NativeAkashicEntity(akashic_entity)) in akashic_entities.iter() {
        akashic_entity.modified();
        commands.entity(entity).remove::<RequestModifyTarget>();
    }
}


fn invalidate_entity(
    mut commands: Commands,
    akashic_entities: Query<(Entity, &NativeAkashicLabel), With<RequestInvalidateTarget>>,
) {
    for (entity, NativeAkashicLabel(akashic_entity)) in akashic_entities.iter() {
        akashic_entity.invalidate();
        commands.entity(entity).remove::<(
            RequestModifyTarget,
            RequestInvalidateTarget,
        )>();
    }
}