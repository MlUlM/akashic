use bevy::app::{App, Update};
use bevy::prelude::Plugin;

use crate::input::point::down::{PointDownPlugin, subscribe_click_event};
use crate::input::point::r#move::{pop_mouse_move_queue, subscribe_pointermove_event};
use crate::input::point::up::{pop_mouse_released_queue, subscribe_pointerup_event};
use crate::input::wheel_scroll::WheelScrollPlugin;

mod point;

#[allow(unused)]
mod touch;
mod wheel_scroll;


pub struct AkashicInputPlugin;

impl Plugin for AkashicInputPlugin {
    fn build(&self, app: &mut App) {
        subscribe_click_event(app);
        subscribe_pointermove_event(app);
        subscribe_pointerup_event(app);

        app
            .add_plugins((
                PointDownPlugin,
                WheelScrollPlugin
            ))
            .add_systems(Update, (
                pop_mouse_move_queue,
                pop_mouse_released_queue,
            ));
    }
}




