use bevy::ecs::system::SystemParam;
use bevy::log::info;
use bevy::math::{Vec3, Vec3Swizzles};
use bevy::prelude::{Camera, Commands, Component, Entity, GlobalTransform, NonSendMut, Query, Res};
use bevy_rapier3d::pipeline::QueryFilter;
use bevy_rapier3d::plugin::RapierContext;

use bevy_akashic::component::object2d::touchable::Touchable;
use bevy_akashic::event::point::AkashicPointEventBase;

use crate::input::akashic_pointer::AkashicPontEventStorage;

pub(crate) fn update_hit_rapiers<E: AkashicPointEventBase + Clone, C: From<E> + Component>(
    mut commands: Commands,
    mut storage: NonSendMut<AkashicPontEventStorage<E>>,
    rapier: RapierParam,
) {
    for event in storage.iter().cloned() {
        let Some(target_entity) = rapier.find(event.pointer_location()) else { continue; };
        commands
            .entity(target_entity)
            .insert(C::from(event));
    }

    **storage = storage
        .iter()
        .cloned()
        .filter(|event| rapier.find(event.pointer_location()).is_none())
        .collect();
}


#[derive(SystemParam)]
pub(crate) struct RapierParam<'w, 's> {
    rapier_context: Option<Res<'w, RapierContext>>,
    targets: Query<'w, 's, (Entity, &'static Touchable)>,
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
}


impl<'w, 's> RapierParam<'w, 's> {
    pub fn find(&self, position: Vec3) -> Option<Entity> {
        let (camera, transform) = self.camera.get_single().ok()?;

        find_pick_entity(
            &self.rapier_context,
            &self.targets,
            camera,
            transform,
            position,
        )
    }
}


fn find_pick_entity<'w, 's>(
    rapier_context: &Option<Res<RapierContext>>,
    targets: &Query<'w, 's, (Entity, &'static Touchable)>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec3,
) -> Option<Entity> {
    let mut viewport_pos = position.xy();

    if let Some(viewport) = &camera.viewport {
        viewport_pos -= viewport.physical_position.as_vec2();
    }

    let ray = camera.viewport_to_world(camera_transform, viewport_pos)?;
    let rapier_context = rapier_context.as_ref()?;

    rapier_context
        .cast_ray_and_get_normal(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            QueryFilter::new().predicate(&|entity| {
                targets
                    .get(entity)
                    .map(|(_, touchable)| touchable.0)
                    .unwrap_or(false)
            }),
        )
        .map(|(target, _)| {
            info!("find entity = {target:?}");

            target
        })
}

