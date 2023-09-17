use bevy::app::App;
use bevy::prelude::Plugin;
use akashic_rs::game::GAME;
use crate::prelude::player_id::SelfPlayerId;

pub struct PlayerIdPlugin;


impl Plugin for PlayerIdPlugin {
    fn build(&self, app: &mut App) {
        if let Some(player_id) = GAME.self_id(){
            app.insert_resource(SelfPlayerId(Some(player_id)));
        }else {
            app.insert_resource(SelfPlayerId(None));
        }
    }
}