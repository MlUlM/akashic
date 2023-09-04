use bevy::app::{App, Update};
use bevy::prelude::{Event, EventWriter, Plugin, Res};

use crate::trigger::point_down::{PointDown, AkashicEventQueue, ScenePointDown};

pub struct PointDownPlugin;


impl Plugin for PointDownPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicEventQueue<PointDown>>()
            .init_resource::<AkashicEventQueue<ScenePointDown>>()
            .add_event::<PointDown>()
            .add_event::<ScenePointDown>()
            .add_systems(Update, (
                read_event_system::<PointDown>,
                read_event_system::<ScenePointDown>
            ));
    }
}


fn read_event_system<T: Event>(
    mut ew: EventWriter<T>,
    queue: Res<AkashicEventQueue<T>>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(event);
    }
}