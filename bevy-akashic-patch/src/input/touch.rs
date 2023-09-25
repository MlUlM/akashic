use bevy::input::touch::{ForceTouch, TouchPhase};
use bevy::math::Vec2;
use bevy::prelude::{Deref, EventWriter, NonSend, TouchInput};
use web_sys::TouchEvent;

use akashic::console_log;
use bevy_akashic::event::AkashicEventQueue;

use crate::winit::AkashicSurface;

#[derive(Deref, Debug)]
pub(crate) struct HtmlTouchEvent(TouchEvent);


pub(crate) fn subscribe_touchstart_event(app: &mut bevy::prelude::App) {
    let click_queue = bevy_akashic::event::AkashicEventQueue::<HtmlTouchEvent>::default();
    app.insert_non_send_resource(click_queue.clone());
    let cb = wasm_bindgen::closure::Closure::<dyn Fn(TouchEvent)>::new(move |event: TouchEvent| {
        event.prevent_default();
        click_queue.push(HtmlTouchEvent(event));
    });

    use wasm_bindgen::JsCast;
    app.world.non_send_resource::<AkashicSurface>().canvas()
        .set_ontouchstart(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
}




pub(crate) fn pop_touch_event_queue(
    mut ew: EventWriter<TouchInput>,
    queue: NonSend<AkashicEventQueue<HtmlTouchEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        console_log!("+++++++ {event:?}");
        let touch_list = event.target_touches();
        if touch_list.is_undefined() {
            continue;
        }

        console_log!("{touch_list:?}");

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