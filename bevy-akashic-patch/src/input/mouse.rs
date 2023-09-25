use bevy::math::Vec2;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{DomRect, PointerEvent};

pub mod click;
pub mod r#move;
pub mod up;

#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(js_name = canvasRect)]
    fn canvas_rect() -> DomRect;
}

pub(crate) fn convert_to_position(
    event: &PointerEvent,
) -> Vec2 {
    let rect = canvas_rect();
    Vec2::new(event.client_x() as f32 - rect.left() as f32, event.client_y() as f32 - rect.top() as f32)
}


#[macro_use]
pub(crate) mod macros {
    macro_rules! subscribe_html_event {
        ($event_name: ident, $html_event_type: ident, $event_type: ident) => {
            paste::paste!{
                pub(crate) fn [<subscribe_ $event_name _event>](
                    app: &mut bevy::prelude::App
                ) {
                    let click_queue = bevy_akashic::event::AkashicEventQueue::<$event_type>::default();
                    app.insert_non_send_resource(click_queue.clone());

                    let cb = wasm_bindgen::closure::Closure::<dyn Fn($html_event_type)>::new(move |event| {
                        click_queue.push($event_type(event));
                    });
                    use wasm_bindgen::JsCast;
                    app
                        .world
                        .non_send_resource::<$crate::winit::AkashicSurface>()
                        .canvas()
                        .[<set_on $event_name>](Some(cb.as_ref().unchecked_ref()));
                    cb.forget();
                }
            }
        };
    }


    pub(crate) use subscribe_html_event;
}