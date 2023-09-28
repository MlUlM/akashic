use bevy::app::App;
use bevy::prelude::Plugin;

use crate::input::touch::cancel::TouchCancelPlugin;
use crate::input::touch::end::TouchEndPlugin;
use crate::input::touch::moved::TouchMovedPlugin;
use crate::input::touch::start::TouchStartPlugin;

pub mod start;
pub mod moved;
pub mod end;
pub mod cancel;


pub struct TouchPlugins;


impl Plugin for TouchPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TouchStartPlugin,
                TouchMovedPlugin,
                TouchEndPlugin,
                TouchCancelPlugin
            ));
    }
}
