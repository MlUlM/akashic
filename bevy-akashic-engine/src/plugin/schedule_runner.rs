use bevy::app::{App, Plugin};

use akashic_rs::prelude::UpdateHandler;
use akashic_rs::prelude::GAME;

use crate::plugin::scene::NativeScene;

pub struct AkashicScheduleRunnerPlugin;


impl Plugin for AkashicScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_non_send_resource(NativeScene(GAME.scene()))
            .set_runner(move |mut app| {
                app
                    .world
                    .non_send_resource::<NativeScene>()
                    .on_update()
                    .add(move || {
                        app.update();
                    });
            });
    }
}



