use bevy::math::Vec2;
use bevy::window::Window;
use web_sys::{PointerEvent, window};
use bevy_akashic::resource::game::GameInfo;

pub mod click;
pub mod r#move;
pub mod up;


pub(crate) fn convert_to_position(
    event: &PointerEvent,
    game_info: &GameInfo
) -> Vec2{
    return Vec2::new(event.offset_x() as f32, event.offset_y() as f32);

    let x = event.x() as f32 - game_info.half_width() ;
    let y = game_info.half_height()  - event.y() as f32;
    Vec2::new(x, y)
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