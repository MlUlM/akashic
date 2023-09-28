use bevy::app::{App, PreUpdate};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{CursorLeft, PrimaryWindow};
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct PointerLeftPlugin;

impl Plugin for PointerLeftPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointerleave_event(app);

        app.add_systems(PreUpdate, pop_event_queue);
    }
}

#[derive(Deref)]
struct HtmlPointerLeaveEvent(PointerEvent);


subscribe_html_event!(pointerleave, PointerEvent, HtmlPointerLeaveEvent);

fn pop_event_queue(
    mut ew: EventWriter<CursorLeft>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlPointerLeaveEvent>>,
) {
    while queue.pop_front().is_some() {
        ew.send(CursorLeft {
            window: window.single(),
        });
    }
}
