use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::prelude::{Deref, Entity, EventWriter, MouseButton, NonSend, Query, Res, TouchInput, With};
use bevy::window::PrimaryWindow;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;
use bevy_akashic::resource::game::GameInfo;

use crate::input::mouse::convert_to_position;
use crate::input::mouse::macros::subscribe_html_event;

#[derive(Deref)]
pub(crate) struct HtmlClickEvent(PointerEvent);


subscribe_html_event!(click, PointerEvent, HtmlClickEvent);

pub(crate) fn pop_click_event_queue(
    mut ew: EventWriter<MouseButtonInput>,
    mut touch_writer: EventWriter<TouchInput>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlClickEvent>>,
    game_info: Res<GameInfo>,
) {
    while let Some(event) = queue.pop_front() {
        let button = event.button();
        touch_writer.send(TouchInput {
            id: event.pointer_id() as u64,
            position: convert_to_position(&event, &game_info),
            force: None,
            phase: TouchPhase::Started,
        });

        ew.send(MouseButtonInput {
            button: convert_to_mouse_button(button),
            state: ButtonState::Pressed,
            window: window.single(),
        });
    }
}


pub(crate) fn convert_to_mouse_button(raw: i16) -> MouseButton {
    match raw {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
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