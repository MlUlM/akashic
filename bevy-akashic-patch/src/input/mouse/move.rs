use bevy::input::mouse::MouseMotion;
use bevy::input::touch::TouchPhase;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, TouchInput, With};
use bevy::window::{CursorMoved, PrimaryWindow};
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::mouse::convert_to_position;
use crate::input::mouse::macros::subscribe_html_event;

#[derive(Deref)]
pub(crate) struct HtmlMouseMoveEvent(PointerEvent);


subscribe_html_event!(pointermove, PointerEvent, HtmlMouseMoveEvent);




pub(crate) fn pop_mouse_move_queue(
    mut ew: EventWriter<CursorMoved>,
    mut touch_writer: EventWriter<TouchInput>,
    mut moved: EventWriter<MouseMotion>,
    mut window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlMouseMoveEvent>>,
) {
    while let Some(event) = queue.pop_front()
    {
        let pos = convert_to_position(&event);

        let entity = window.single_mut();

        moved.send(MouseMotion {
            delta: pos,
        });
        touch_writer.send(TouchInput {
            phase: TouchPhase::Moved,
            position: pos,
            force: None,
            id: event.pointer_id() as u64,
        });
        ew.send(CursorMoved {
            window: entity,
            position: pos,
        })
    }
}