use bevy::app::{App, Plugin, PostUpdate, PreUpdate};
use bevy::prelude::{Added, Commands, Entity, EventWriter, Query, Transform};

use akashic_rs::prelude::GAME;

use crate::component::previous_transform::PreviousTransform;
use crate::plugin::render::SceneModifiedEvent;
use crate::prelude::AkashicEntityId;

pub struct AkashicTransformPlugin;

impl Plugin for AkashicTransformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, insert_previous_transform_system)
            .add_systems(PostUpdate, transform_system);
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


fn transform_system(
    mut transforms: Query<(&AkashicEntityId, &Transform, &mut PreviousTransform)>,
    mut ew: EventWriter<SceneModifiedEvent>,
) {
    for (AkashicEntityId(id), transform, mut previous_transform) in transforms.iter_mut() {
        if previous_transform.eq(transform) {
            continue;
        }

        let Some(entity) = GAME.scene().find_child(*id) else { continue; };
        let previous = &previous_transform.0;
        update_positions(&entity, previous, transform);
        update_angle(&entity, previous, transform);

        *previous_transform = PreviousTransform(*transform);
        ew.send(SceneModifiedEvent);
    }
}


fn update_positions(
    entity: &akashic_rs::prelude::Entity,
    previous: &Transform,
    current: &Transform,
) {
    let pos = &current.translation;
    if previous.translation.x != pos.x {
        entity.set_x(pos.x);
    }
    if previous.translation.y != pos.y {
        entity.set_y(pos.y);
    }
}


fn update_angle(
    _entity: &akashic_rs::prelude::Entity,
    previous: &Transform,
    current: &Transform,
) {
    let (_, previous) = previous.rotation.to_axis_angle();
    let (_, current) = current.rotation.to_axis_angle();

    if previous != current {}
}
