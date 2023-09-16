use std::sync::{Arc, RwLock};

use bevy::app::{PluginGroup, PluginGroupBuilder};

use akashic_rs::prelude::SceneParameterObject;

use crate::plugin::append::AkashicAppendEntityPlugin;
use crate::plugin::asset::AkashicAssetPlugin;
use crate::plugin::event::{PointDownPlugin, PointMovePlugin, PointUpPlugin};
use crate::plugin::game_info::GameInfoPlugin;
use crate::plugin::game_state::AkashicGameScorePlugin;
use crate::plugin::join::AkashicJoinEventPlugin;
use feed::label::AkashicLabelPlugin;
use crate::plugin::modify::AkashicModifyPlugin;
use crate::plugin::player_id::PlayerIdPlugin;
use crate::plugin::random::AkashicRandomPlugin;
use crate::plugin::despawn::AkashicDespawnPlugin;
use crate::plugin::feed::entity_object2d::AkashicEntityObject2DPlugin;

use crate::plugin::system_set::AkashicSystemSetPlugin;
use crate::prelude::AkashicScheduleRunnerPlugin;


pub mod scheduler;
pub mod despawn;
pub mod event;
pub mod join;
pub mod player_id;
pub mod game_info;
pub mod asset;
pub mod random;
pub mod game_state;
pub mod system_set;
pub mod modify;
pub mod append;
pub mod feed;


pub mod prelude {
    pub use crate::plugin::{
        AkashicMinimumPlugins,
        despawn::AkashicDespawnPlugin,
        join::AkashicJoinEventPlugin,
        scheduler::AkashicScheduleRunnerPlugin,
    };
}


#[derive(Default)]
pub struct AkashicMinimumPlugins;


impl PluginGroup for AkashicMinimumPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AkashicSystemSetPlugin)
            .add(AkashicAssetPlugin)
            .add(AkashicGameScorePlugin)
            .add(AkashicRandomPlugin)
            .add(GameInfoPlugin)
            .add(PlayerIdPlugin)
            .add(PointDownPlugin)
            .add(PointMovePlugin)
            .add(PointUpPlugin)
            .add(AkashicJoinEventPlugin)
            .add(AkashicAppendEntityPlugin)
            .add(AkashicEntityObject2DPlugin)
            .add(AkashicLabelPlugin)
            .add(AkashicDespawnPlugin)
            .add(AkashicModifyPlugin)
            .add(AkashicScheduleRunnerPlugin)
    }
}


#[derive(Clone, Default)]
pub(crate) struct SharedSceneParameter(Arc<RwLock<SceneParameterObject>>);


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
