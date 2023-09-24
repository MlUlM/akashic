use bevy::input::mouse::MouseMotion;
use bevy::input::touch::TouchPhase;
use bevy::math::IVec2;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, Res, TouchInput, With};
use bevy::window::{CursorMoved, PrimaryWindow};
use web_sys::PointerEvent;

use akashic::console_log;
use bevy_akashic::event::AkashicEventQueue;
use bevy_akashic::resource::game::GameInfo;

use crate::input::mouse::convert_to_position;
use crate::input::mouse::macros::subscribe_html_event;

#[derive(Deref)]
pub(crate) struct HtmlMouseMoveEvent(PointerEvent);


subscribe_html_event!(pointermove, PointerEvent, HtmlMouseMoveEvent);




pub(crate) fn pop_mouse_move_queue(
    mut ew: EventWriter<CursorMoved>,
    mut touch_writer: EventWriter<TouchInput>,
    mut moved: EventWriter<MouseMotion>,
    mut window: Query<(Entity, &mut bevy::prelude::Window), With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlMouseMoveEvent>>,
    game_info: Res<GameInfo>,
) {
    while let Some(event) = queue.pop_front()
    {
        let pos = convert_to_position(&event, &game_info);

        let (entity, mut window) = window.single_mut();

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