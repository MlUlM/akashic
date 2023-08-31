use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::{App, Plugin, PreUpdate};
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, ResMut};
use wasm_bindgen::JsValue;

use akashic_rs::game::GAME;
use akashic_rs::prelude::{OnLoadHandler, UpdateHandler};
use akashic_rs::scene::param::SceneParameterObject;
use akashic_rs::scene::Scene;

use crate::plugin::SceneLoadState;

pub struct AkashicSchedulerPlugin;


impl Plugin for AkashicSchedulerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                PreUpdate,
                (
                    load_scene_event
                ).run_if(in_state(SceneLoadState::Loading)))
            .set_runner(|mut app| {
                let scene = Scene::new(SceneParameterObject::default());
                scene.on_load().add(|_| {
                    IS_LOADED.store(true, Ordering::Relaxed);
                });
                scene.on_update().add(move || {
                    app.update();
                });
                
                GAME.push_scene(scene.clone(), JsValue::UNDEFINED);
            });
    }
}


static IS_LOADED: AtomicBool = AtomicBool::new(false);

fn load_scene_event(mut state: ResMut<NextState<SceneLoadState>>) {
    if IS_LOADED.load(Ordering::Relaxed) {
        state.set(SceneLoadState::Loaded);
    }
}
