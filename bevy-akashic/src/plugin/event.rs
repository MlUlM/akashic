use bevy::ecs::query::WorldQuery;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{App, CalculatedClip, Commands, Component, ComputedVisibility, Entity, EventWriter, GlobalTransform, IntoSystemConfigs, Node, NonSend, Query, Res, TouchInput, With};
use bevy::ui::{RelativeCursorPosition, UiStack};
use bevy::window::{PrimaryWindow, Window};

use akashic::event::point::point_move::PointMoveEvent;
use akashic::event::point::point_up::PointUpEvent;
use akashic::prelude::{EntityObject2D, PointDownCaptureHandler, PointDownEvent};
use akashic::trigger::point::point_move::PointMoveCaptureHandler;
use akashic::trigger::point::point_up::PointUpCaptureHandler;
use akashic::trigger::PointEventBase;

use crate::component::AkashicEntityId;
use crate::component::object2d::touchable::Touchable;
use crate::event::AkashicEventQueue;
use crate::event::point_down::OnPointDown;
use crate::event::point_move::OnPointMove;
use crate::plugin::event::point_down::RapierParam;
use crate::plugin::scene::NativeScene;
use crate::prelude::point_up::OnPointUp;
use crate::prelude::scene::GameScene;
use crate::resource::game::GameInfo;

mod point_down;

macro_rules! trigger_plugin {
    ($plugin_name: ident, $native_event: ident, $component: ident, $scene_trigger_name: ident) => {
        pub struct $plugin_name;

        impl bevy::prelude::Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app
                    .init_non_send_resource::<AkashicEventQueue<$native_event>>()
                    .add_systems(bevy::prelude::PreStartup, |
                        queue: NonSend<AkashicEventQueue<$native_event>>,
                        scene: NonSend<NativeScene>,
                    |{
                        let queue = queue.clone();
                        scene
                            .$scene_trigger_name()
                            .add(move |event| {
                                queue.push(event);
                            });
                    })
                    .add_systems(bevy::prelude::PreUpdate, |
                        mut commands: Commands,
                        queue: NonSend<AkashicEventQueue<$native_event>>,
                        akashic_entities: Query<(Entity, &AkashicEntityId)>,
                        window: Query<Entity, With<PrimaryWindow>>,
                        game_info: Res<GameInfo>,
                        rapier: RapierParam
                    |{
                        while let Some(event) = queue.pop_front() {
                            let target_id = event.target().map(|akashic_entity| akashic_entity.id());
                            let pos = Vec3::new(event.point().x(), event.point().y(), 0.);
                            let c = $component::new(event, game_info.half_width(), game_info.half_height());
                            if let Some(target_entity) = find_point_event_target(&akashic_entities, target_id)
                                .or_else(||rapier.find(pos))
                            {
                                commands
                                    .entity(target_entity)
                                    .insert(c);
                            } else {
                                commands
                                    .entity(window.single())
                                    .insert(c);
                            }
                        }
                    })
                    .add_systems(bevy::prelude::Last, (
                        remove_point_component_system::<$component>
                    ).in_set(crate::plugin::system_set::AkashicSystemSet::PointEvents));
            }
        }
    };
}


trigger_plugin!(PointDownPlugin, PointDownEvent, OnPointDown, on_point_down_capture);
trigger_plugin!(PointUpPlugin, PointUpEvent, OnPointUp, on_point_up_capture);
trigger_plugin!(PointMovePlugin, PointMoveEvent, OnPointMove, on_point_move_capture);


//
// #[derive(WorldQuery)]
// #[world_query(mutable)]
// pub struct NodeQuery {
//     entity: Entity,
//     node: &'static Node,
//     global_transform: &'static GlobalTransform,
//     touchable: &'static Touchable,
//     relative_cursor_position: Option<&'static mut RelativeCursorPosition>,
//     calculated_clip: Option<&'static CalculatedClip>,
//     computed_visibility: Option<&'static ComputedVisibility>,
// }
//
// fn ui_picking(
//     pointer: Vec3,
//     node_query: &Query<NodeQuery>,
//     ui_stack: &Res<UiStack>,
// ) -> Option<Entity> {
//     let mut hovered_nodes = ui_stack
//         .uinodes
//         .iter()
//         // reverse the iterator to traverse the tree from closest nodes to furthest
//         .rev()
//         .filter_map(|entity| {
//             if let Ok(node) = node_query.get(*entity) {
//                 if !node.touchable.0 {
//                     return None;
//                 }
//
//                 if let Some(computed_visibility) = node.computed_visibility {
//                     if !computed_visibility.is_visible() {
//                         return None;
//                     }
//                 }
//
//                 let position = node.global_transform.translation();
//
//                 let ui_position = position.truncate();
//
//                 let extents = node.node.size() / 2.0;
//                 let mut min = ui_position - extents;
//                 if let Some(clip) = node.calculated_clip {
//                     min = Vec2::max(min, clip.clip.min);
//                 }
//
//                 let relative_cursor_position = Vec2::new(
//                     (pointer.x - min.x) / node.node.size().x,
//                     (pointer.y - min.y) / node.node.size().y,
//                 );
//
//                 if (0.0..1.).contains(&relative_cursor_position.x)
//                     && (0.0..1.).contains(&relative_cursor_position.y)
//                 {
//                     Some(*entity)
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<Entity>>()
//         .into_iter();
//
//     hovered_nodes.next()
// }

fn remove_point_component_system<P: Component>(
    mut commands: Commands,
    point_event_targets: Query<Entity, With<P>>,
) {
    for entity in point_event_targets.iter() {
        commands.entity(entity).remove::<P>();
    }
}

pub(crate) fn find_point_event_target(
    akashic_entities: &Query<(Entity, &AkashicEntityId)>,
    target_id: Option<isize>,
) -> Option<Entity> {
    let target_id = target_id?;
    akashic_entities
        .iter()
        .find_map(|(entity, id)| {
            if id.0 == target_id {
                Some(entity)
            } else {
                None
            }
        })
}

