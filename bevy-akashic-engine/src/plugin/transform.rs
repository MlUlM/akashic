use std::f32::consts::PI;

use bevy::app::{App, Plugin, PreUpdate};
use bevy::prelude::{Added, Commands, Entity, Query, Res, Transform};

use crate::component::entity_size::{AkashicEntitySize, PreviousAkashicEntitySize};
use crate::component::previous_transform::PreviousTransform;
use crate::prelude::AkashicEntityId;

use super::render::AkashicEntityMap;


pub struct AkashicTransformPlugin;

impl Plugin for AkashicTransformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, insert_previous_transform_system);
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


pub(crate) fn transform_system(
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

