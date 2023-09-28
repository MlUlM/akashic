use bevy::app::App;
use bevy::input::touch::{ForceTouch, TouchPhase};
use bevy::math::Vec2;
use bevy::prelude::{Deref, EventWriter, NonSend, Plugin, TouchInput};
use web_sys::TouchEvent;

use bevy_akashic::event::AkashicEventQueue;
use crate::input::pointer::macros::subscribe_html_event;

pub struct TouchCancelPlugin;


impl Plugin for TouchCancelPlugin {
    fn build(&self, app: &mut App) {
        subscribe_touchcancel_event(app);

        app.add_systems(bevy::prelude::PreUpdate, pop_event_queue);
    }
}



subscribe_html_event!(touchcancel, TouchEvent, HtmlTouchCancelEvent);


#[derive(Deref, Debug)]
struct HtmlTouchCancelEvent(TouchEvent);



fn pop_event_queue(
    mut ew: EventWriter<TouchInput>,
    queue: NonSend<AkashicEventQueue<HtmlTouchCancelEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        let touch_list = event.target_touches();
        if touch_list.is_undefined() {
            continue;
        }

        for i in 0..touch_list.length() {
            let touch = touch_list.get(i).unwrap();
            ew.send(TouchInput {
                phase: TouchPhase::Canceled,
                position: Vec2::new(touch.client_x() as f32, touch.client_y() as f32),
                force: if touch.force() == 0. { None } else { Some(ForceTouch::Normalized(touch.force() as f64)) },
                id: touch.identifier() as u64,
            });
        }
    }
}