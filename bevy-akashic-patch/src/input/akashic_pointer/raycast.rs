use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{Camera, Commands, Component, Entity, GlobalTransform, NonSendMut, Query};
use bevy::utils::petgraph::visit::Walker;
use bevy_mod_raycast::prelude::{Raycast, RaycastVisibility};
use bevy_mod_raycast::Ray3d;

use bevy_akashic::event::point::AkashicPointEventBase;
use bevy_akashic::prelude::object2d::touchable::Touchable;

use crate::input::akashic_pointer::AkashicPontEventStorage;

pub(crate) fn update_hit_raycasts<E: AkashicPointEventBase + Clone, C: From<E> + Component>(
    mut commands: Commands,
    mut storage: NonSendMut<AkashicPontEventStorage<E>>,
    mut raycast: Raycast,
    camera: Query<(&Camera, &GlobalTransform)>,
    pickables: Query<&Touchable>,
) {
    let mut no_hits: Vec<E> = Vec::with_capacity(storage.len());

    for event in storage.iter().cloned() {
        let (camera, camera_transform) = camera.single();
        if let Some(hits) = find_hits(camera, camera_transform, &mut raycast, event.pointer_location().xy(), &pickables) {
            for entity in hits {
                commands.entity(entity).insert(C::from(event.clone()));
            }
        } else {
            no_hits.push(event);
        }
    }

    **storage = no_hits;
}


fn find_hits(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    raycast: &mut Raycast,
    point: Vec2,
    pickables: &Query<&Touchable>,
) -> Option<Vec<Entity>> {
    let ray = camera.viewport_to_world(camera_transform, point)?;

    let settings = bevy_mod_raycast::system_param::RaycastSettings {
        visibility: RaycastVisibility::MustBeVisibleAndInView,
        filter: &|_| true,
        early_exit_test: &|entity_hit| {
            pickables
                .get(entity_hit)
                .is_ok_and(|touchable| touchable.0)
        },
    };

    let hits = raycast
        .cast_ray(Ray3d::new(ray.origin, ray.direction), &settings)
        .iter()
        .map(|(entity, _)| entity)
        .copied()
        .collect::<Vec<_>>();


    if hits.is_empty() {
        None
    } else {
        Some(hits)
    }
}