use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, FromWorld, Msaa, Plugin, Query, With};
use bevy::window::{RawHandleWrapper, Window};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};
use web_sys::window;

use akashic_rs::game::GAME;
use akashic_rs::prelude::SpriteBuilder;

use crate::command::IntoBundle;
use crate::plugin::akashic_3d::{AkashicSurface, canvas_only};

pub struct AkashicWinitPlugin;

impl Plugin for AkashicWinitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        // let event_loop = EventLoopBuilder::new().build();
        let surface = canvas_only(GAME.width() as u32, GAME.height() as u32);
        window().unwrap().document().unwrap().body().unwrap().append_child(&surface.canvas()).unwrap();
        // let window = WindowBuilder::new()
        //     .with_canvas(Some(surface.canvas()))
        //     .build(&event_loop)
        //     .unwrap();
        let sprite = SpriteBuilder::new(surface.clone())
            .width(GAME.width())
            .height(GAME.height())
            .build();
        app.world.spawn(sprite.into_bundle());

        let mut state: SystemState<
            Query<Entity, With<Window>>
        > = SystemState::from_world(&mut app.world);

        let primary = state.get(&app.world);
        let canvas = surface.canvas();

        canvas.set_attribute("data-raw-handle", "1").unwrap();
        let mut window_handle = WebWindowHandle::empty();
        window_handle.id = 1;

        let display_handle = WebDisplayHandle::empty();
        app.world
            .entity_mut(primary.single())
            .insert(RawHandleWrapper {
                window_handle: RawWindowHandle::Web(window_handle),
                display_handle: RawDisplayHandle::Web(display_handle),
            });
        app.world.insert_non_send_resource(AkashicSurface(surface));
    }
}