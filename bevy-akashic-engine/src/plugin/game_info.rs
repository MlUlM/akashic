use bevy::app::App;
use bevy::prelude::Plugin;
use crate::resource::game::GameInfo;

pub struct GameInfoPlugin;

impl Plugin for GameInfoPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInfo>();
    }
}
