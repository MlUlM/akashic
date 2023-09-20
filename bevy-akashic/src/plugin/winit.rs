use std::mem::forget;
use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, FromWorld, Msaa, Plugin, Query, With};
use bevy::window::{PrimaryWindow, RawHandleWrapper, Window};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use web_sys::window;
use winit::event_loop::EventLoopBuilder;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::window::WindowBuilder;
use akashic_rs::game::GAME;
use akashic_rs::prelude::SpriteBuilder;
use crate::command::IntoBundle;
use crate::plugin::akashic_3d::{canvas, canvas_only};

pub struct AkashicWinitPlugin;

impl Plugin for AkashicWinitPlugin {
    fn build(&self, app: &mut App) {
      app.insert_resource(Msaa::Off);
        let event_loop = EventLoopBuilder::new().build();
        let surface = canvas_only(GAME.width() as u32, GAME.height() as u32);
        // window().unwrap().document().unwrap().body().unwrap().append_child(&surface.canvas()).unwrap();
        let window = WindowBuilder::new()
            .with_canvas(Some(surface.canvas()))
            .build(&event_loop)
            .unwrap();
        let sprite = SpriteBuilder::new(surface.clone())
            .width(GAME.width())
            .height(GAME.height())
            .build();
        app.world.spawn(sprite.into_bundle());
     forget(surface);
        let mut state: SystemState<
            Query<Entity, With<Window>>
        > = SystemState::from_world(&mut app.world);

        let primary = state.get(&app.world);

        app.world
            .entity_mut(primary.single())
            .insert(RawHandleWrapper{
               window_handle: window.raw_window_handle(),
                display_handle: window.raw_display_handle()
            });
    }
}