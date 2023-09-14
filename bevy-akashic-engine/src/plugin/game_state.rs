use bevy::app::{App, Plugin};
use crate::resource::game_state::AkashicGameState;

pub struct GameStatePlugin;


impl Plugin for GameStatePlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<AkashicGameState>();
    }
}