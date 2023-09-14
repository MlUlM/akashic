use std::sync::{Arc, RwLock};

use bevy::app::{PluginGroup, PluginGroupBuilder};

use akashic_rs::prelude::SceneParameterObject;

use crate::plugin::asset::AkashicAssetPlugin;
use crate::plugin::event::{PointDownPlugin, PointMovePlugin, PointUpPlugin};
use crate::plugin::game_info::GameInfoPlugin;
use crate::plugin::game_state::GameStatePlugin;
use crate::plugin::join::AkashicJoinEventPlugin;
use crate::plugin::player_id::PlayerIdPlugin;
use crate::plugin::random::AkashicRandomPlugin;
use crate::plugin::render::AkashicRenderPlugin;
use crate::plugin::transform::AkashicTransformPlugin;

pub mod scheduler;
pub mod render;
pub mod transform;
pub mod event;
pub mod join;
pub mod player_id;
pub mod game_info;
pub mod asset;
pub mod random;
pub mod game_state;


pub mod prelude {
    pub use crate::plugin::{
        AkashicMinimumPlugins,
        join::AkashicJoinEventPlugin,
        render::AkashicRenderPlugin,
        scheduler::AkashicSchedulerPlugin,
        transform::AkashicTransformPlugin,
    };
}


#[derive(Default)]
pub struct AkashicMinimumPlugins;


impl PluginGroup for AkashicMinimumPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AkashicAssetPlugin)
            .add(GameInfoPlugin)
            .add(PlayerIdPlugin)
            .add(PointDownPlugin)
            .add(PointMovePlugin)
            .add(PointUpPlugin)
            .add(AkashicRenderPlugin)
            .add(AkashicTransformPlugin)
            .add(AkashicJoinEventPlugin)
            .add(AkashicRandomPlugin)
            .add(GameStatePlugin)
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
