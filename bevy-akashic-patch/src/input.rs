use bevy::app::{App, Update};
use bevy::prelude::Plugin;

use crate::input::mouse::click::{pop_click_event_queue, subscribe_click_event};
use crate::input::mouse::r#move::{pop_mouse_move_queue, subscribe_pointermove_event};
use crate::input::mouse::up::{pop_mouse_released_queue, subscribe_pointerup_event};

mod mouse;

#[allow(unused)]
mod touch;


pub struct AkashicInputPlugin;

impl Plugin for AkashicInputPlugin {
    fn build(&self, app: &mut App) {
        subscribe_click_event(app);
        subscribe_pointermove_event(app);
        subscribe_pointerup_event(app);
        // subscribe_touchstart_event(app);

        app
            .add_systems(Update, (
                pop_click_event_queue,
                pop_mouse_move_queue,
                pop_mouse_released_queue,
                // pop_touch_event_queue
            ));
    }
}



