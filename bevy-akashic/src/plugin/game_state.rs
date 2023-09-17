use bevy::app::{App, Plugin};
use crate::resource::game_score::GameScore;

pub struct AkashicGameScorePlugin;


impl Plugin for AkashicGameScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>();
    }
}