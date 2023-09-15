use std::f32::consts::PI;

use bevy::app::{App, Last, Plugin, PreUpdate};
use bevy::prelude::{Added, Commands, Component, Deref, Entity, IntoSystemConfigs, Query, Res, Transform};

use crate::component::entity_size::AkashicEntitySize;
use crate::plugin::akashic_entity_map::AkashicEntityMap;
use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::AkashicEntityId;


pub struct AkashicTransformPlugin;

impl Plugin for AkashicTransformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, (
                insert_previous_transform_system,
                insert_previous_size_system
            ))
            .add_systems(Last, (
                modify_transform_system
            ).in_set(AkashicSystemSet::ModifyToNativeAkashicEntities));
    }
}

#[allow(clippy::type_complexity)]
fn insert_previous_transform_system(
    mut commands: Commands,
    akashic_entities: Query<(Entity, &Transform), (Added<Transform>, Added<AkashicEntityId>)>,
) {
    for (entity, transform) in akashic_entities.iter() {
        commands.entity(entity).insert(PreviousTransform::from(*transform));
    }
}


#[allow(clippy::type_complexity)]
fn insert_previous_size_system(
    mut commands: Commands,
    akashic_entities: Query<(Entity, &AkashicEntitySize), (Added<AkashicEntitySize>, Added<AkashicEntityId>)>,
) {
    for (entity, size) in akashic_entities.iter() {
        commands.entity(entity).insert(PreviousAkashicEntitySize::from(*size));
    }
}


fn modify_transform_system(
    mut transforms: Query<(
        Entity,
        &Transform,
        &AkashicEntitySize,
        &mut PreviousTransform,
        &mut PreviousAkashicEntitySize
    )>,
    entity_map: Res<AkashicEntityMap>,
) {
    for (
        entity,
        transform,
        size,
        mut prev_transform,
        mut prev_size
    ) in transforms.iter_mut() {
        if prev_transform.eq(transform) && prev_size.eq(size) {
            continue;
        }

        let Some(akashic_entity) = entity_map.0.get(&entity) else { continue; };
        let (_, rad) = transform.rotation.to_axis_angle();
        let angle = rad * 180. / PI;

        *prev_transform = PreviousTransform(*transform);
        *prev_size = PreviousAkashicEntitySize(*size);

        akashic_entity.update(
            transform.translation.x,
            transform.translation.y,
            angle,
            size.x,
            size.y,
        );
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
struct PreviousTransform(pub(crate) Transform);

impl PartialEq<Transform> for PreviousTransform {
    #[inline(always)]
    fn eq(&self, other: &Transform) -> bool {
        &self.0 == other
    }
}


impl From<Transform> for PreviousTransform {
    fn from(value: Transform) -> Self {
        Self(value)
    }
}


#[derive(Component, Debug, Copy, Clone, PartialEq, Deref)]
struct PreviousAkashicEntitySize(pub(crate) AkashicEntitySize);


impl PartialEq<AkashicEntitySize> for PreviousAkashicEntitySize {
    #[inline(always)]
    fn eq(&self, other: &AkashicEntitySize) -> bool {
        &self.0 == other
    }
}


impl From<AkashicEntitySize> for PreviousAkashicEntitySize {
    fn from(value: AkashicEntitySize) -> Self {
        Self(value)
    }
}