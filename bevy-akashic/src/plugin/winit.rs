use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, FromWorld, Plugin, Query, With};
use bevy::window::{RawHandleWrapper, Window};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};

use akashic::game::GAME;

use crate::plugin::akashic_3d::{AkashicSurface, canvas_only};

pub struct AkashicWinitPlugin;


impl Plugin for AkashicWinitPlugin {
    fn build(&self, app: &mut App) {
        let surface = canvas_only(GAME.width() as u32, GAME.height() as u32);


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
        app.world.insert_non_send_resource(AkashicSurface(surface));
    }
}