use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, FromWorld, Query, Window, With};
use bevy::window::RawHandleWrapper;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};


use crate::winit::{AkashicSurface, create_screen_surface};

pub struct AkashicWindowPlugin;

impl Plugin for AkashicWindowPlugin {
    fn build(&self, app: &mut App) {
        let akashic_surface = create_screen_surface();
        app.insert_non_send_resource(AkashicSurface(akashic_surface));
        let mut state: SystemState<
            Query<Entity, With<Window>>
        > = SystemState::from_world(&mut app.world);

        let mut query = state.get_mut(&mut app.world);
        let primary= query.single_mut();

        let mut window_raw_handle = WebWindowHandle::empty();
        window_raw_handle.id = 1;

        app.world
            .entity_mut(primary)
            .insert(RawHandleWrapper {
                window_handle: RawWindowHandle::Web(window_raw_handle),
                display_handle: RawDisplayHandle::Web(WebDisplayHandle::empty()),
            });
    }
}