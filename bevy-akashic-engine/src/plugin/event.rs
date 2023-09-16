use bevy::prelude::{App, Commands, Component, Entity, NonSend, Query, With, IntoSystemConfigs};
use akashic_rs::event::point::point_move::PointMoveEvent;
use akashic_rs::event::point::point_up::PointUpEvent;

use akashic_rs::prelude::{EntityObject2D, PointDownCaptureHandler, PointDownEvent};
use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;
use akashic_rs::trigger::PointEventBase;

use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::point_down::OnPointDown;
use crate::event::point_move::OnPointMove;
use crate::prelude::point_up::OnPointUp;
use crate::prelude::scene::GameScene;
use crate::plugin::scene::NativeScene;

macro_rules! trigger_plugin {
    ($plugin_name: ident, $native_event: ident, $component: ident, $scene_trigger_name: ident) => {
        pub struct $plugin_name;

        impl bevy::prelude::Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app
                    .init_non_send_resource::<AkashicEventQueue<$native_event>>()
                    .add_systems(bevy::prelude::Startup, |
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
                        scene: Query<Entity, With<GameScene>>,
                    |{
                        while let Some(event) = queue.pop_front() {
                            let target_id = event.target().map(|akashic_entity| akashic_entity.id());
                            if let Some(target_entity) = find_point_event_target(&akashic_entities, target_id) {
                                commands
                                    .entity(target_entity)
                                    .insert($component::new(event));
                            } else {
                                commands
                                    .entity(scene.single())
                                    .insert($component::new(event));
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


fn remove_point_component_system<P: Component>(
    mut commands: Commands,
    point_event_targets: Query<Entity, With<P>>
){
    for entity in point_event_targets.iter(){
        commands.entity(entity).remove::<P>();
    }
}


fn find_point_event_target(
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

