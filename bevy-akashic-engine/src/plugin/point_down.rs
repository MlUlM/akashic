use bevy::app::{App, Update};
use bevy::prelude::{EventWriter, Plugin, Res};

use crate::trigger::point_down::{PointDown, PointDownQueue};

pub struct PointDownPlugin;


impl Plugin for PointDownPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PointDownQueue>()
            .add_event::<PointDown>()
            .add_systems(Update, read_point_down_queue);
    }
}


fn read_point_down_queue(
    mut ew: EventWriter<PointDown>,
    queue: Res<PointDownQueue>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(event);
    }
}