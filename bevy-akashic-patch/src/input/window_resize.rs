use bevy::app::{App, PreUpdate};
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{PrimaryWindow, WindowResized};
use web_sys::UiEvent;
use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;
use crate::winit::AkashicSurface;

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    fn build(&self, app: &mut App) {
        subscribe_resize_event(app);

        app.add_systems(PreUpdate, pop_event_queue);
    }
}

#[derive(Deref)]
struct HtmlWindowResizeEvent(UiEvent);


subscribe_html_event!(resize, UiEvent, HtmlWindowResizeEvent);

fn pop_event_queue(
    mut ew: EventWriter<WindowResized>,
    akashic_surface: NonSend<AkashicSurface>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlWindowResizeEvent>>,
) {
    if queue.is_empty() {
        return;
    }

    let canvas = akashic_surface.canvas();
    while queue.pop_front().is_some() {
        ew.send(WindowResized {
            width: canvas.width() as f32,
            height: canvas.height() as f32,
            window: window.single(),
        });
    }
}
