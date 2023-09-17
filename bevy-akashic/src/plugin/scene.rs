use bevy::app::{App, Plugin, PreStartup};
use bevy::prelude::{Commands, Deref, NonSend};

use akashic_rs::game::GAME;
use akashic_rs::prelude::Scene;

use crate::prelude::scene::GameScene;

pub struct AkashicScenePlugin;

impl Plugin for AkashicScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_non_send_resource(NativeScene(GAME.scene()))
            .add_systems(PreStartup, spawn_game_scene_system);
    }
}


fn spawn_game_scene_system(
    mut commands: Commands,
    scene: NonSend<NativeScene>,
) {
    commands.spawn(GameScene(scene.0.clone()));
}

#[derive(Debug, Deref)]
pub(crate) struct NativeScene(pub(crate) Scene);