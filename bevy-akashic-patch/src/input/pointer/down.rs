use bevy::app::{App, PreUpdate};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{Deref, Entity, EventWriter, MouseButton, NonSend, Plugin, Query, With};
use bevy::window::PrimaryWindow;
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct PointerDownPlugin;

impl Plugin for PointerDownPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointerdown_event(app);

        app.add_systems(PreUpdate, pop_event_queue);
    }
}

#[derive(Deref)]
pub(crate) struct HtmlPointerDownEvent(PointerEvent);


subscribe_html_event!(pointerdown, PointerEvent, HtmlPointerDownEvent);

fn pop_event_queue(
    mut ew: EventWriter<MouseButtonInput>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlPointerDownEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        let button = event.button();

        ew.send(MouseButtonInput {
            button: convert_to_mouse_button(button),
            state: ButtonState::Pressed,
            window: window.single(),
        });
    }
}


pub(crate) fn convert_to_mouse_button(raw: i16) -> MouseButton {
    match raw {
        1 => MouseButton::Left,
        4 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => {
            if let Ok(raw) = u16::try_from(raw) {
                MouseButton::Other(raw)
            } else {
                MouseButton::Left
            }
        }
    }
}