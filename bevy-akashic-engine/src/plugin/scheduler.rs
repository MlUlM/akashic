use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::Vec2;
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, ResMut, World};
use wasm_bindgen::JsValue;

use akashic_rs::prelude::{OnLoadHandler, PointDownCaptureHandler, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::Scene;

use crate::plugin::{SceneLoadState, SharedSceneParameter};
use crate::trigger::point_down::{AkashicEventQueue, ScenePointDown};

#[derive(Default)]
pub struct AkashicSchedulerPlugin(pub(crate) SharedSceneParameter);


impl Plugin for AkashicSchedulerPlugin {
    fn build(&self, app: &mut App) {
        let param = self.0.clone();

        app
            .add_systems(
                PreUpdate,
                (
                    load_scene_event
                ).run_if(in_state(SceneLoadState::Loading)))
            .set_runner(move |mut app| {
                let scene = Scene::new(param.param());

                on_point_down_capture(&scene, &mut app.world);

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

fn on_point_down_capture(
    scene: &impl PointDownCaptureHandler,
    world: &mut World,
) {
    let point_down_queue = world.resource::<AkashicEventQueue<ScenePointDown>>().clone();

    scene.on_point_down_capture().add(move |e| {
        let point = e.point();
        point_down_queue.push(ScenePointDown {
            point: Vec2::new(point.x(), point.y())
        })
    });
}