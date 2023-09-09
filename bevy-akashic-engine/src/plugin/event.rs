use bevy::prelude::{App, Event, EventWriter, Res};

use crate::event::AkashicEventQueue;
use crate::event::point_down::PointDown;
use crate::event::point_move::{PointMoveEvent, ScenePointMoveEvent};
use crate::event::point_up::{PointUpEvent, ScenePointUpEvent};
use crate::prelude::point_down::ScenePointDown;

macro_rules! trigger_plugin {
    ($name: ident, $event: ident $(, $scene_event: ident)?) => {
        pub struct $name;

        impl bevy::prelude::Plugin for $name {
            fn build(&self, app: &mut App) {
                app
                    .init_resource::<AkashicEventQueue<$event>>()
                    $(.init_resource::<AkashicEventQueue<$scene_event>>())?
                    .add_event::<$event>()
                    $(.add_event::<$scene_event>())?
                    .add_systems(bevy::prelude::Update, (
                        read_event_system::<$event>,
                        $(read_event_system::<$scene_event>)?
                    ));
            }
        }
    };
}


trigger_plugin!(PointDownPlugin, PointDown, ScenePointDown);
trigger_plugin!(PointUpPlugin, PointUpEvent, ScenePointUpEvent);
trigger_plugin!(PointMovePlugin, PointMoveEvent, ScenePointMoveEvent);

pub(crate) fn read_event_system<T: Event>(
    mut ew: EventWriter<T>,
    queue: Res<AkashicEventQueue<T>>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(event);
    }
}