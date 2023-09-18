use bevy::app::{App, Plugin};

use akashic_rs::prelude::UpdateHandler;

use crate::plugin::scene::NativeScene;

pub struct AkashicScheduleRunnerPlugin;


impl Plugin for AkashicScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .set_runner(move |mut app| {
                app
                    .world
                    .non_send_resource::<NativeScene>()
                    .on_update()
                    .add(move || {
                        let mut finished_and_setup_done = false;
                        if !finished_and_setup_done {
                            if !app.ready(){
                                return;
                            }
                            finished_and_setup_done = true;
                            app.finish();
                            app.cleanup();
                        }

                        if app.ready(){
                            app.update();
                        }
                    });
            });
    }
}



