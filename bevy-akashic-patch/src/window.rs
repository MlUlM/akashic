use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, FromWorld, Plugin, Query, With};
use bevy::window::{RawHandleWrapper, Window};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};
use wasm_bindgen::prelude::wasm_bindgen;


pub struct AkashicWindowPlugin;


impl Plugin for AkashicWindowPlugin {
    fn build(&self, app: &mut App) {
        create_screen_surface();

        let mut state: SystemState<
            Query<Entity, With<Window>>
        > = SystemState::from_world(&mut app.world);

        let primary = state.get(&app.world);
        let mut window_handle = WebWindowHandle::empty();
        window_handle.id = 1;

        let display_handle = WebDisplayHandle::empty();
        app.world
            .entity_mut(primary.single())
            .insert(RawHandleWrapper {
                window_handle: RawWindowHandle::Web(window_handle),
                display_handle: RawDisplayHandle::Web(display_handle),
            });
    }
}


#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = g)]
    fn create_screen_surface() -> akashic::asset::surface::Surface;
}