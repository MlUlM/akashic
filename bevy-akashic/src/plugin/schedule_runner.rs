use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::{App, AppExit, Plugin};
use bevy::ecs::event::ManualEventReader;
use bevy::prelude::Events;

use akashic::game::GAME;

use akashic::prelude::UpdateHandler;

use crate::plugin::scene::NativeScene;

pub struct AkashicScheduleRunnerPlugin;


impl Plugin for AkashicScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

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

                        if let Some(app_exit_events) = app.world.get_resource::<Events<AppExit>>() {
                            if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                                GAME.pop_scene();
                                return;
                            }
                        }

                        if app.ready() {
                            app.update();
                        }
                    });
            });
    }
}



