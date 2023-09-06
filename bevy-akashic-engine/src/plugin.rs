use std::sync::{Arc, RwLock};

use bevy::app::{App, Plugin};
use bevy::prelude::States;
use akashic_rs::prelude::SceneParameterObject;

use crate::asset::AkashicAssetServer;
use crate::asset::game::GameInfo;
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

    Startup
}


#[derive(Default)]
pub struct AkashicPlugin(SharedSceneParameter);


impl AkashicPlugin {
    #[inline]
    pub fn new(scene_param: SceneParameterObject) -> Self {
        Self(SharedSceneParameter::new(scene_param))
    }
}

impl Plugin for AkashicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SceneLoadState>()
            .init_resource::<AkashicAssetServer>()
            .init_resource::<GameInfo>()
            .add_plugins(PointDownPlugin)
            .add_plugins(AkashicRenderPlugin)
            .add_plugins(AkashicSchedulerPlugin(self.0.clone()));
    }
}


#[derive(Clone, Default)]
pub struct SharedSceneParameter(Arc<RwLock<SceneParameterObject>>);


impl SharedSceneParameter {
    #[inline]
    pub fn new(scene_param: SceneParameterObject) -> Self {
        Self(Arc::new(RwLock::new(scene_param)))
    }


    #[inline]
    pub fn param(&self) -> SceneParameterObject {
        self.0.read().unwrap().clone()
    }
}


unsafe impl Send for SharedSceneParameter {}

unsafe impl Sync for SharedSceneParameter {}
