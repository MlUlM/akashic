use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::MutexGuard;

use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Deref, DerefMut, in_state, IntoSystemConfigs, NextState, Res, ResMut, Resource, World};
use wasm_bindgen::JsValue;

use akashic_rs::entity::E;
use akashic_rs::prelude::{OnLoadHandler, PointDownCaptureHandler, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::Scene;
use akashic_rs::trigger::join::JoinHandler;
use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;
use akashic_rs::trigger::PointEventBase;

use crate::asset::AkashicAssetServer;
use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::message::RegisterAkashicMessageFn;
use crate::event::point_down::{PointDown, ScenePointDown};
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::ScenePointUpEvent;
use crate::extensions::AsVec3;
use crate::plugin::{SceneLoadState, SharedSceneParameter};
use crate::prelude::player_id::PlayerId;
use crate::prelude::point_move::ScenePointMoveEvent;
use crate::resource::join::{JoinedAsListener, JoinedAsStreamer, JoinStatus, JoinStatusResource};


pub struct AkashicSchedulerPlugin(pub(crate) SharedSceneParameter, pub(crate) Vec<RegisterAkashicMessageFn>);


impl Plugin for AkashicSchedulerPlugin {
    fn build(&self, app: &mut App) {
        let param = self.0.clone();
        let fs = self.1.clone();
        let join_status = JoinStatusResource::default();

        app
            .insert_resource(join_status.clone())
            .add_systems(PreUpdate, (
                init_asset_server
            ).run_if(in_state(SceneLoadState::Loaded)))
            .add_systems(
                PreUpdate,
                (
                    loading_scene_system
                ).run_if(in_state(SceneLoadState::Loading)))
            .set_runner(move |mut app| {
                let scene = Scene::new(param.param());

                on_point_down_capture(&scene, &mut app.world);
                on_point_up_capture(&scene, &mut app.world);
                on_point_move_capture(&scene, &mut app.world);
                register_on_join(join_status);

                for f in fs.iter() {
                    f(&mut app, &scene);
                }

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

fn loading_scene_system(
    mut state: ResMut<NextState<SceneLoadState>>,
    mut commands: Commands,
    join_status: Res<JoinStatusResource>,
) {
    if IS_LOADED.load(Ordering::Relaxed) && join_status.lock().not_undefined() {
        match join_status.lock().clone() {
            JoinStatus::Streamer(player_id) => {
                commands.insert_resource(JoinedAsStreamer(PlayerId(player_id)));
            }
            JoinStatus::Listener(player_id) => {
                commands.insert_resource(JoinedAsListener(PlayerId(player_id)));
            }
            _ => {}
        }

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
    let queue = world.resource::<AkashicEventQueue<PointDown>>().clone();

    scene.on_point_down_capture().add(move |e| {
        let point = e.point();
        if let Some(target) = e.target() {
            queue.push(PointDown {
                entity_id: AkashicEntityId(target.id()),
                point: Vec2::new(point.x(), point.y()),
            })
        } else {
            point_down_queue.push(ScenePointDown {
                point: Vec2::new(point.x(), point.y())
            })
        }
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
                point: point.as_vec3(),
                start_delta: e.start_delta().as_vec3(),
                prev_delta: e.prev_delta().as_vec3(),
            });
        } else {
            point_down_queue.push(ScenePointMoveEvent {
                point: point.as_vec3(),
                start_delta: e.start_delta().as_vec3(),
                prev_delta: e.prev_delta().as_vec3(),
            });
        }
    });
}

fn register_on_join(status: JoinStatusResource) {
    GAME.on_join().add(move |join_event| {
        let Some(streamer_id) = join_event.player().id() else { return; };
        if let Some(self_id) = GAME.self_id() {
            // JoinEventが発火されるのはニコ生の場合配信者だけらしいため、
            // 自身のIDと同じ場合は配信者となる
            if self_id == streamer_id {
                *status.lock() = JoinStatus::Streamer(self_id);
            } else {
                *status.lock() = JoinStatus::Listener(self_id);
            }
        } else {
            // 自身のIDが存在しない場合Node側
            *status.lock() = JoinStatus::NodeServer;
        }
    });
}
