use bevy::ecs::system::SystemParam;
use bevy::math::{Vec3, Vec3Swizzles};
use bevy::prelude::{Camera, Entity, GlobalTransform, Query, Res};
use bevy_rapier3d::pipeline::QueryFilter;
use bevy_rapier3d::prelude::RapierContext;

use crate::prelude::object2d::touchable::Touchable;

// fn receive_point_down_event(
//     mut commands: Commands,
//     queue: NonSend<AkashicEventQueue<PointDownEvent>>,
//     akashic_entities: Query<(Entity, &AkashicEntityId)>,
//     scene: Query<Entity, With<GameScene>>,
//     game_info: Res<GameInfo>,
// ) {
//     while let Some(event) = queue.pop_front() {
//         let target_id = event.target().map(|akashic_entity| akashic_entity.id());
//         let pos = Vec3::new(event.point().x(), event.point().y(), 0.);
//         let c = OnPointDown::new(event, game_info.half_width(), game_info.half_height());
//
//         if let Some(target_entity) = find_point_event_target(&akashic_entities, target_id) {
//             commands
//                 .entity(target_entity)
//                 .insert(c);
//         } else {
//             commands
//                 .entity(scene.single())
//                 .insert(c);
//         }
//     }
// }

// fn pop_click_event_queue(
//     mut button_input: EventWriter<MouseButtonInput>,
//     mut touch: EventWriter<TouchInput>,
//     window: &Window,
//     event: &PointDownEvent,
// ) {
//     let point = event.point();
//     let button = event.button();
//     touch.send(TouchInput {
//         id: event.pointer_id() as u64,
//         position: Vec2::new(point.x(), point.y()),
//         force: None,
//         phase: TouchPhase::Started,
//     });
//
//     button_input.send(MouseButtonInput {
//         button: convert_to_mouse_button(button),
//         state: ButtonState::Pressed,
//         window: window.single(),
//     });
// }

#[derive(SystemParam)]
pub struct RapierParam<'w, 's> {
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
    targets:  &Query<'w, 's, (Entity, &'static Touchable)>,
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
                    .map(|(_, touchable)|touchable.0)
                    .unwrap_or(false)
            }),
        )
        .map(|(target, _)| {
            target
        })
}