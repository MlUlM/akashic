use bevy::app::{App, Plugin};
use bevy::prelude::States;
use crate::plugin::point_down::PointDownPlugin;
use crate::plugin::render::AkashicRenderPlugin;

use crate::plugin::scheduler::AkashicSchedulerPlugin;

mod scheduler;
mod point_down;
mod render;


#[derive(Eq, PartialEq, Hash, States, Default, Debug, Clone)]
pub enum SceneLoadState {
    #[default]
    Loading,

    Loaded,
}

pub struct AkashicPlugin;

impl Plugin for AkashicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SceneLoadState>()
            .add_plugins(PointDownPlugin)
            .add_plugins(AkashicRenderPlugin)
            .add_plugins(AkashicSchedulerPlugin);
    }
}
