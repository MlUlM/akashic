use bevy::app::{App, PreUpdate};
use bevy::log::info;
use bevy::prelude::{Deref, Entity, EventWriter, NonSend, Plugin, Query, With};
use bevy::window::{PrimaryWindow, WindowFocused};
use web_sys::FocusEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct WindowFocusPlugin;

impl Plugin for WindowFocusPlugin {
    fn build(&self, app: &mut App) {
        subscribe_focus_event(app);
        subscribe_blur_event(app);

        app
            .add_systems(PreUpdate, (
                pop_focus_queue,
                pop_blur_queue
            ));
    }
}

#[derive(Deref)]
struct HtmlFocusEvent(FocusEvent);

#[derive(Deref)]
struct HtmlBlurEvent(FocusEvent);


subscribe_html_event!(focus, FocusEvent, HtmlFocusEvent);
subscribe_html_event!(blur, FocusEvent, HtmlBlurEvent);


fn pop_focus_queue(
    mut ew: EventWriter<WindowFocused>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlFocusEvent>>,
) {
    while queue.pop_front().is_some() {
        info!("focus");
        ew.send(WindowFocused {
            focused: true,
            window: window.single(),
        });
    }
}


fn pop_blur_queue(
    mut ew: EventWriter<WindowFocused>,
    window: Query<Entity, With<PrimaryWindow>>,
    queue: NonSend<AkashicEventQueue<HtmlBlurEvent>>,
) {
    while queue.pop_front().is_some() {
        ew.send(WindowFocused {
            focused: false,
            window: window.single(),
        });
    }
}


