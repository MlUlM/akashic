use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_akashic::plugin::akashic_3d::is_node;
use crate::input::akashic_pointer::AkashicPointPatchPlugins;

use crate::input::focus::WindowFocusPlugin;
use crate::input::pointer::PointerPlugins;
use crate::input::touch::TouchPlugins;
use crate::input::wheel_scroll::WheelScrollPlugin;
use crate::input::window_resize::WindowResizePlugin;

pub mod pointer;
pub mod touch;
pub mod wheel_scroll;
pub mod focus;
pub mod akashic_pointer;
pub mod window_resize;


pub struct AkashicInputPlugin;

impl Plugin for AkashicInputPlugin {
    fn build(&self, app: &mut App) {
        if !is_node() {
            app
            .add_plugins((
                PointerPlugins,
                TouchPlugins,
                AkashicPointPatchPlugins,
                WheelScrollPlugin,
                WindowFocusPlugin,
                WindowResizePlugin
            ));
        }
    }
}




