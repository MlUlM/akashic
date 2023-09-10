use std::sync::{Arc, RwLock};

use bevy::app::{App, Plugin};
use bevy::prelude::{Event, States};
use serde::de::DeserializeOwned;
use serde::Serialize;
use akashic_rs::prelude::SceneParameterObject;

use crate::asset::AkashicAssetServer;
use crate::resource::game::GameInfo;
use crate::event::message::{add_akashic_message_event, RegisterAkashicMessageFn};
use crate::plugin::event::{PointDownPlugin, PointMovePlugin, PointUpPlugin};
use crate::plugin::join::AkashicJoinEventPlugin;
use crate::plugin::render::AkashicRenderPlugin;
use crate::plugin::scheduler::AkashicSchedulerPlugin;
use crate::plugin::transform::AkashicTransformPlugin;

mod scheduler;

pub mod render;
pub mod transform;
pub mod event;
pub mod join;


#[derive(Eq, PartialEq, Hash, States, Default, Debug, Clone)]
pub enum SceneLoadState {
    #[default]
    Loading,

    Loaded,

    Startup,
}


#[derive(Default)]
pub struct AkashicPlugin {
    scene_param: SharedSceneParameter,
    message_event_registers: Vec<RegisterAkashicMessageFn>,
}


impl AkashicPlugin {
    #[inline]
    pub fn new(scene_param: SceneParameterObject) -> Self {
        Self {
            scene_param: SharedSceneParameter::new(scene_param),
            message_event_registers: Vec::new(),
        }
    }


    #[inline]
    pub fn add_message_event<E>(mut self) -> Self
        where E: Event + Serialize + DeserializeOwned
    {
        self.message_event_registers.push(add_akashic_message_event::<E>());
        self
    }
}

impl Plugin for AkashicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SceneLoadState>()
            .init_resource::<AkashicAssetServer>()
            .init_resource::<GameInfo>()
            .add_plugins((
                PointDownPlugin,
                PointUpPlugin,
                PointMovePlugin,
                AkashicRenderPlugin,
                AkashicSchedulerPlugin(self.scene_param.clone(), self.message_event_registers.clone()),
                AkashicTransformPlugin,
                AkashicJoinEventPlugin
            ));
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
