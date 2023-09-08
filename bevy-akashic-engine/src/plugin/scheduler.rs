use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::Vec2;
use bevy::prelude::{Commands, in_state, IntoSystemConfigs, NextState, ResMut, World};
use wasm_bindgen::JsValue;

use akashic_rs::console_log;
use akashic_rs::entity::E;
use akashic_rs::prelude::{OnLoadHandler, PointDownCaptureHandler, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::Scene;
use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;

use crate::asset::AkashicAssetServer;
use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::point_down::ScenePointDown;
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::ScenePointUpEvent;
use crate::extensions::AsVec2;
use crate::plugin::{SceneLoadState, SharedSceneParameter};
use crate::prelude::point_move::ScenePointMoveEvent;

pub struct AkashicSchedulerPlugin(pub(crate) SharedSceneParameter);


impl Plugin for AkashicSchedulerPlugin {
    fn build(&self, app: &mut App) {
        let param = self.0.clone();

        app
            .add_systems(PreUpdate, (
                init_asset_server
            ).run_if(in_state(SceneLoadState::Loaded)))
            .add_systems(
                PreUpdate,
                (
                    load_scene_event
                ).run_if(in_state(SceneLoadState::Loading)))
            .set_runner(move |mut app| {
                let scene = Scene::new(param.param());

                on_point_down_capture(&scene, &mut app.world);
                on_point_up_capture(&scene, &mut app.world);
                on_point_move_capture(&scene, &mut app.world);

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

fn init_asset_server(
    mut state: ResMut<NextState<SceneLoadState>>,
    mut commands: Commands,
) {
    commands.insert_resource(AkashicAssetServer::default());
    state.set(SceneLoadState::Startup);
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


fn on_point_up_capture(
    scene: &impl PointUpCaptureHandler,
    world: &mut World,
) {
    let point_down_queue = world.resource::<AkashicEventQueue<ScenePointUpEvent>>().clone();
    scene.on_point_up_capture().add(move |e| {
        let point = e.point();
        point_down_queue.push(ScenePointUpEvent {
            point: Vec2::new(point.x(), point.y()),
        })
    });
}


fn on_point_move_capture(
    scene: &impl PointMoveCaptureHandler,
    world: &mut World,
) {
    let point_down_queue = world.resource::<AkashicEventQueue<ScenePointMoveEvent>>().clone();
    let queue = world.resource::<AkashicEventQueue<PointMoveEvent>>().clone();
    scene.on_point_move_capture().add(move |e| {
        let point = e.point();
        if let Some(akashic_entity) = e.target() {
            queue.push(PointMoveEvent {
                entity_id: AkashicEntityId(akashic_entity.id()),
                point: point.as_vec2(),
                start_delta: e.start_delta().as_vec2(),
                prev_delta: e.prev_delta().as_vec2(),
            });
        } else {
            point_down_queue.push(ScenePointMoveEvent {
                point: point.as_vec2(),
                start_delta: e.start_delta().as_vec2(),
                prev_delta: e.prev_delta().as_vec2(),
            })
        }
    });
}