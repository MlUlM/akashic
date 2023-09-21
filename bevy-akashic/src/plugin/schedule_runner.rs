use std::sync::atomic::{AtomicBool, Ordering};

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
                        static SETUP_DONE: AtomicBool = AtomicBool::new(false);
                        if !SETUP_DONE.load(Ordering::Relaxed) {
                            if !app.ready() {
                                return;
                            }
                            SETUP_DONE.store(true, Ordering::Relaxed);
                            app.finish();
                            app.cleanup();
                        }

                        if app.ready() {
                            app.update();
                        }
                    });
            });
    }
}



