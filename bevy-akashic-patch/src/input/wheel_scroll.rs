use bevy::app::{App, PreUpdate};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::PrimaryWindow;
use web_sys::WheelEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct WheelScrollPlugin;

impl Plugin for WheelScrollPlugin {
    fn build(&self, app: &mut App) {
        subscribe_wheel_event(app);

        app.add_systems(PreUpdate, pop_wheel_event_queue);
    }
}

#[derive(Deref)]
pub(crate) struct HtmlWheelEvent(WheelEvent);


subscribe_html_event!(wheel, WheelEvent, HtmlWheelEvent);

fn pop_wheel_event_queue(
    mut ew: EventWriter<MouseWheel>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlWheelEvent>>,
) {
    while let Some(event) = queue.pop_front() {

        ew.send(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: event.delta_x() as f32,
            y: -event.delta_y() as f32,
            window: window.single(),
        });
    }
}

