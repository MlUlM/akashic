use bevy::app::{App, Plugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, With};
use bevy::window::{CursorMoved, PrimaryWindow};
use web_sys::PointerEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::convert_to_position;
use crate::input::pointer::macros::subscribe_html_event;


pub struct PointerMovedPlugin;

impl Plugin for PointerMovedPlugin {
    fn build(&self, app: &mut App) {
        subscribe_pointermove_event(app);

        app.add_systems(bevy::prelude::PreUpdate, pop_event_queue);
    }
}


#[derive(Deref)]
struct HtmlMouseMoveEvent(PointerEvent);


subscribe_html_event!(pointermove, PointerEvent, HtmlMouseMoveEvent);


fn pop_event_queue(
    mut ew: EventWriter<CursorMoved>,
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

        ew.send(CursorMoved {
            window: entity,
            position: pos,
        })
    }
}