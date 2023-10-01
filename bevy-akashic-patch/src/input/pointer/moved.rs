use bevy::app::{App, Plugin};
use bevy::input::mouse::MouseMotion;
use bevy::math::{DVec2, Vec2};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Query, With};
use bevy::window::{CursorMoved, PrimaryWindow, Window};
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
    mut window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlMouseMoveEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        moved.send(MouseMotion {
            delta: Vec2::new(event.movement_x() as f32, event.movement_y() as f32)
        });

        let position = convert_to_position(&event);
        let (entity, mut window) = window.single_mut();

        let physical_position = DVec2::new(position.x as f64, position.y as f64);
        window.set_physical_cursor_position(Some(physical_position));

        ew.send(CursorMoved {
            window: entity,
            position: (physical_position / window.resolution.scale_factor()).as_vec2(),
        })
    }
}