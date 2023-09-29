use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use feed::label::AkashicLabelPlugin;

use crate::plugin::append::AkashicAppendEntityPlugin;
use crate::plugin::asset::AkashicAssetServerPlugin;
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

#[allow(unused)]
#[cfg(feature = "3d")]
pub mod akashic_3d;

#[allow(unused)]
#[cfg(feature = "3d")]
pub mod akashic_3d2;
pub mod request_snapshot;


pub mod prelude {
    pub use crate::plugin::{
        AkashicCorePlugins,
        despawn::AkashicDespawnPlugin,
        join::AkashicJoinEventPlugin,
        schedule_runner::AkashicScheduleRunnerPlugin,
    };
}


#[derive(Default)]
pub struct AkashicCorePlugins;


impl Plugin for AkashicCorePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AkashicSystemSetPlugin,
                AkashicScenePlugin,
                AkashicAssetServerPlugin,
                AkashicGameScorePlugin,
                GameInfoPlugin,
                PlayerIdPlugin,
                PointDownPlugin,
                PointMovePlugin,
                PointUpPlugin,
            ))
            .add_plugins((
                AkashicJoinEventPlugin,
                AkashicAppendEntityPlugin,
                AkashicEntityObject2DPlugin,
                AkashicLabelPlugin,
                FilledRectPlugin,
                AkashicDespawnPlugin,
            ));
    }
}

impl PluginGroup for AkashicCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AkashicSystemSetPlugin)
            .add(AkashicScenePlugin)
            .add(AkashicAssetServerPlugin)
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
    }
}



