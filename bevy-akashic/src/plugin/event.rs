use auto_delegate::Delegate;
use bevy::prelude::{App, Commands, Component, Deref, Entity, Event, EventWriter, IntoSystemConfigs, NonSend, Query, With};

use akashic::event::point::point_move::PointMoveEvent;
use akashic::event::point::point_up::PointUpEvent;
use akashic::prelude::{PointDownCaptureHandler, PointDownEvent};
use akashic::trigger::point::point_move::PointMoveCaptureHandler;
use akashic::trigger::point::point_up::PointUpCaptureHandler;

use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::point::down::OnPointDown;
use crate::event::point::event_inner::{PointDeltaEventInner, PointEventInner};
use crate::event::point::up::OnPointUp;
use crate::plugin::scene::NativeScene;
use crate::prelude::point::r#move::OnPointMove;
use crate::unsafe_impl_all_synchronization;

#[derive(Event, Deref, Clone, Debug, Delegate)]
#[to(AkashicPointEventBase, PointEventBase)]
pub struct AkashicPointDownEvent(PointEventInner<PointDownEvent>);
unsafe_impl_all_synchronization!(AkashicPointDownEvent);
impl AkashicPointDownEvent {
    #[inline(always)]
    fn new(native_event: PointDownEvent) -> Self {
        Self(PointEventInner::new(native_event))
    }
}


#[derive(Event, Deref, Clone, Debug, Delegate)]
#[to(AkashicPointEventBase, PointEventBase)]
pub struct AkashicPointMoveEvent(PointDeltaEventInner<PointMoveEvent>);
unsafe_impl_all_synchronization!(AkashicPointMoveEvent);
impl AkashicPointMoveEvent {
    #[inline(always)]
    fn new(native_event: PointMoveEvent) -> Self {
        Self(PointDeltaEventInner::new(native_event))
    }
}


#[derive(Event, Deref, Clone, Debug, Delegate)]
#[to(AkashicPointEventBase, PointEventBase)]
pub struct AkashicPointUpEvent(PointDeltaEventInner<PointUpEvent>);
unsafe_impl_all_synchronization!(AkashicPointUpEvent);
impl AkashicPointUpEvent {
    #[inline(always)]
    fn new(native_event: PointUpEvent) -> Self {
        Self(PointDeltaEventInner::new(native_event))
    }
}




macro_rules! point_plugin {
    (
        $plugin_name: ident,
        $native_event: ident,
        $bevy_event: ident,
        $component: ident,
        $scene_trigger_name: ident
    ) => {
        pub struct $plugin_name;

        impl bevy::prelude::Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app
                    .init_non_send_resource::<AkashicEventQueue<$native_event>>()
                    .add_event::<$bevy_event>()
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
                   .add_systems(bevy::prelude::First, |
                        mut ew: EventWriter<$bevy_event>,
                        queue: NonSend<AkashicEventQueue<$native_event>>,
                    |{
                        while let Some(event) = queue.pop_front() {
                            ew.send($bevy_event::new(event));
                        }
                    })
                    .add_systems(bevy::prelude::Last, (
                        remove_point_component_system::<$component>
                    ).in_set(crate::plugin::system_set::AkashicSystemSet::PointEvents));
            }
        }
    };
}


point_plugin!(PointDownPlugin, PointDownEvent, AkashicPointDownEvent, OnPointDown, on_point_down_capture);
point_plugin!(PointMovePlugin, PointMoveEvent, AkashicPointMoveEvent, OnPointMove, on_point_move_capture);
point_plugin!(PointUpPlugin, PointUpEvent, AkashicPointUpEvent, OnPointUp,  on_point_up_capture);

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

