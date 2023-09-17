use bevy::app::{PluginGroup, PluginGroupBuilder};

use feed::label::AkashicLabelPlugin;

use crate::plugin::append::AkashicAppendEntityPlugin;
use crate::plugin::asset::AkashicAssetPlugin;
use crate::plugin::despawn::AkashicDespawnPlugin;
use crate::plugin::event::{PointDownPlugin, PointMovePlugin, PointUpPlugin};
use crate::plugin::feed::entity_object2d::AkashicEntityObject2DPlugin;
use crate::plugin::feed::filled_rect::FilledRectPlugin;
use crate::plugin::game_info::GameInfoPlugin;
use crate::plugin::game_state::AkashicGameScorePlugin;
use crate::plugin::join::AkashicJoinEventPlugin;
use crate::plugin::player_id::PlayerIdPlugin;
use crate::plugin::random::AkashicRandomPlugin;
use crate::plugin::scene::AkashicScenePlugin;
use crate::plugin::system_set::AkashicSystemSetPlugin;
use crate::prelude::AkashicScheduleRunnerPlugin;

pub mod schedule_runner;
pub mod despawn;
pub mod event;
pub mod join;
pub mod player_id;
pub mod game_info;
pub mod asset;
pub mod random;
pub mod game_state;
pub mod system_set;

pub mod append;
pub mod feed;
pub mod scene;


pub mod prelude {
    pub use crate::plugin::{
        AkashicMinimumPlugins,
        despawn::AkashicDespawnPlugin,
        join::AkashicJoinEventPlugin,
        schedule_runner::AkashicScheduleRunnerPlugin,
    };
}


#[derive(Default)]
pub struct AkashicMinimumPlugins;


impl PluginGroup for AkashicMinimumPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AkashicSystemSetPlugin)
            .add(AkashicScenePlugin)
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
            .add(FilledRectPlugin)
            .add(AkashicDespawnPlugin)
            .add(AkashicScheduleRunnerPlugin)
    }
}

