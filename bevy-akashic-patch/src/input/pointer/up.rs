use bevy::app::{App, Plugin};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, With};
use bevy::window::PrimaryWindow;
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::down::convert_to_mouse_button;
use crate::input::pointer::macros::subscribe_html_event;


pub struct PointerUpPlugin;

impl Plugin for PointerUpPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointerup_event(app);

        app.add_systems(bevy::prelude::PreUpdate, pop_event_queue);
    }
}


#[derive(Deref)]
struct HtmlPointerUpEvent(PointerEvent);

subscribe_html_event!(pointerup, PointerEvent, HtmlPointerUpEvent);



fn pop_event_queue(
    mut ew: EventWriter<MouseButtonInput>,
    queue: NonSend<AkashicEventQueue<HtmlPointerUpEvent>>,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(MouseButtonInput {
            state: ButtonState::Released,
            button: convert_to_mouse_button(event.button()),
            window: window.single(),
        });
    }
}
