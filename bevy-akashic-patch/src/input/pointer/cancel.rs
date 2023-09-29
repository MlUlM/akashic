use bevy::app::{App, PreUpdate};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{CursorLeft, PrimaryWindow};
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct PointerCancelPlugin;

impl Plugin for PointerCancelPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointercancel_event(app);

        app.add_systems(PreUpdate, pop_event_queue);
    }
}

#[derive(Deref)]
struct HtmlPointerCancelEvent(PointerEvent);


subscribe_html_event!(pointercancel, PointerEvent, HtmlPointerCancelEvent);

fn pop_event_queue(
    mut ew: EventWriter<CursorLeft>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlPointerCancelEvent>>,
) {
    while queue.pop_front().is_some() {
        ew.send(CursorLeft {
            window: window.single(),
        });
    }
}

