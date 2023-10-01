use bevy::app::{App, PreUpdate};
use bevy::log::info;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{PrimaryWindow, WindowBackendScaleFactorChanged, WindowResized, WindowScaleFactorChanged};
use web_sys::{UiEvent, window};

use bevy_akashic::event::AkashicEventQueue;

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


fn subscribe_resize_event(
    app: &mut bevy::prelude::App
) {
    let click_queue = bevy_akashic::event::AkashicEventQueue::<HtmlWindowResizeEvent>::default();
    app.insert_non_send_resource(click_queue.clone());

    let cb = wasm_bindgen::closure::Closure::<dyn Fn(UiEvent)>::new(move |event| {
        click_queue.push(HtmlWindowResizeEvent(event));
    });
    use wasm_bindgen::JsCast;
    window()
        .unwrap()
        .set_onresize(Some(cb.as_ref().unchecked_ref()));

    cb.forget();
}

fn pop_event_queue(
    mut ew: EventWriter<WindowResized>,
    mut backend_scale_writer: EventWriter<WindowBackendScaleFactorChanged>,
    mut scale_writer: EventWriter<WindowScaleFactorChanged>,
    akashic_surface: NonSend<AkashicSurface>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlWindowResizeEvent>>,
) {
    if queue.is_empty() {
        return;
    }

    let canvas = akashic_surface.canvas();
    let rect = canvas.get_bounding_client_rect();
    let per = web_sys::window().unwrap().device_pixel_ratio();

    while queue.pop_front().is_some() {
        backend_scale_writer.send(WindowBackendScaleFactorChanged {
            window: window.single(),
            scale_factor: per,
        });

        scale_writer.send(WindowScaleFactorChanged {
            scale_factor: per,
            window: window.single(),
        });

        ew.send(WindowResized {
            width: rect.width() as f32,
            height: rect.height() as f32,
            window: window.single(),
        });
    }
}
