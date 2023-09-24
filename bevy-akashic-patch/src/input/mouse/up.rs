use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, Res, TouchInput, With};
use bevy::window::PrimaryWindow;
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;
use bevy_akashic::resource::game::GameInfo;

use crate::input::mouse::click::convert_to_mouse_button;
use crate::input::mouse::convert_to_position;
use crate::input::mouse::macros::subscribe_html_event;

#[derive(Deref)]
pub(crate) struct MouseReleasedEvent(PointerEvent);

subscribe_html_event!(pointerup, PointerEvent, MouseReleasedEvent);



pub(crate) fn pop_mouse_released_queue(
    mut ew: EventWriter<MouseButtonInput>,
    mut touch_writer: EventWriter<TouchInput>,
    queue: NonSend<AkashicEventQueue<MouseReleasedEvent>>,
    window: Query<Entity, With<PrimaryWindow>>,
    game_info: Res<GameInfo>,
) {
    while let Some(event) = queue.pop_front() {
        touch_writer.send(TouchInput {
            id: event.pointer_id() as u64,
            position: convert_to_position(&event, &game_info),
            phase: TouchPhase::Ended,
            force: None,
        });

        ew.send(MouseButtonInput {
            state: ButtonState::Released,
            button: convert_to_mouse_button(event.button()),
            window: window.single(),
        });
    }
}
