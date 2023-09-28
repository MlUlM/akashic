use bevy::app::App;
use bevy::prelude::Plugin;
use crate::input::akashic_pointer::AkashicPointerPlugins;

use crate::input::focus::WindowFocusPlugin;
use crate::input::pointer::PointerPlugins;
use crate::input::touch::TouchPlugins;
use crate::input::wheel_scroll::WheelScrollPlugin;

pub mod pointer;
pub mod touch;
pub mod wheel_scroll;
pub mod focus;
pub mod akashic_pointer;


pub struct AkashicInputPlugin;

impl Plugin for AkashicInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PointerPlugins,
                TouchPlugins,
                AkashicPointerPlugins,
                WheelScrollPlugin,
                WindowFocusPlugin,
            ));
    }
}




