use bevy::app::{App, PreUpdate};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{CursorEntered, PrimaryWindow};
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct PointerEnterPlugin;

impl Plugin for PointerEnterPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointerenter_event(app);

        app.add_systems(PreUpdate, pop_event_queue);
    }
}

#[derive(Deref)]
struct HtmlPointerEnterEvent(PointerEvent);


subscribe_html_event!(pointerenter, PointerEvent, HtmlPointerEnterEvent);

fn pop_event_queue(
    mut ew: EventWriter<CursorEntered>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlPointerEnterEvent>>,
) {
    while queue.pop_front().is_some() {
        ew.send(CursorEntered {
            window: window.single(),
        });
    }
}
