use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use bevy::app::{App, Plugin};
use akashic_rs::console_log;

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
                        static  finished_and_setup_done: AtomicBool = AtomicBool::new(false);
                        if !finished_and_setup_done.load(Ordering::Relaxed) {
                            if !app.ready() {
                                return;
                            }
                            finished_and_setup_done.store(true, Ordering::Relaxed);
                            app.finish();
                            app.cleanup();
                        }

                        static  DADA: AtomicBool = AtomicBool::new(true);
                        if app.ready()  {
                            DADA.store(false, Relaxed);

                            console_log!("UPDATE GAME");
                            app.update();
                        }
                    });
            });
    }
}



