use bevy::app::App;
use bevy::input::touch::{ForceTouch, TouchPhase};
use bevy::math::Vec2;
use bevy::prelude::{Deref, EventWriter, NonSend, Plugin, TouchInput};
use web_sys::TouchEvent;

use bevy_akashic::event::AkashicEventQueue;

use crate::input::pointer::macros::subscribe_html_event;

pub struct TouchStartPlugin;


impl Plugin for TouchStartPlugin {
    fn build(&self, app: &mut App) {
        subscribe_touchstart_event(app);

        app.add_systems(bevy::prelude::PreUpdate, pop_event_queue);
    }
}



subscribe_html_event!(touchstart, TouchEvent, HtmlTouchStartEvent);


#[derive(Deref, Debug)]
struct HtmlTouchStartEvent(TouchEvent);


fn pop_event_queue(
    mut ew: EventWriter<TouchInput>,
    queue: NonSend<AkashicEventQueue<HtmlTouchStartEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        let touch_list = event.target_touches();
        if touch_list.is_undefined() {
            continue;
        }

        for i in 0..touch_list.length() {
            let touch = touch_list.get(i).unwrap();

            ew.send(TouchInput {
                phase: TouchPhase::Started,
                position: Vec2::new(touch.client_x() as f32, touch.client_y() as f32),
                force: if touch.force() == 0. { None } else { Some(ForceTouch::Normalized(touch.force() as f64)) },
                id: touch.identifier() as u64,
            });
        }
    }
}