use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, Commands, IntoSystemConfigs, NextState, Res, ResMut, Resource, World,
};
use wasm_bindgen::JsValue;

use akashic_rs::entity::E;
use akashic_rs::prelude::Scene;
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::{OnLoadHandler, PointDownCaptureHandler, UpdateHandler};

use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;
use akashic_rs::trigger::PointEventBase;

use crate::asset::AkashicAssetServer;
use crate::component::AkashicEntityId;
use crate::event::message::RegisterAkashicMessageFn;
use crate::event::point_down::{PointDown, ScenePointDown};
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::ScenePointUpEvent;
use crate::event::AkashicEventQueue;
use crate::extensions::AsVec3;
use crate::plugin::{SceneLoadState, SharedSceneParameter};

use crate::prelude::point_move::ScenePointMoveEvent;

use crate::SharedObject;

pub struct AkashicSchedulerPlugin(
    pub(crate) SharedSceneParameter,
    pub(crate) Vec<RegisterAkashicMessageFn>,
);

impl Plugin for AkashicSchedulerPlugin {
    fn build(&self, app: &mut App) {
        let param = self.0.clone();
        let fs = self.1.clone();
        let scene_loaded_flag = SceneLoadedFlag::default();

        app
            .insert_resource(scene_loaded_flag.clone())
            .add_systems(
                PreUpdate,
                (init_asset_server).run_if(in_state(SceneLoadState::Loaded)),
            )
            .add_systems(
                PreUpdate,
                (loading_scene_system).run_if(in_state(SceneLoadState::Loading)),
            )
            .set_runner(move |mut app| {
                let scene = Scene::new(param.param());

                on_point_down_capture(&scene, &mut app.world);
                on_point_up_capture(&scene, &mut app.world);
                on_point_move_capture(&scene, &mut app.world);

                for f in fs.iter() {
                    f(&mut app, &scene);
                }

                scene.on_load().add(move |_| {
                    scene_loaded_flag.set_loaded();
                });

                scene
                    .on_update()
                    .add(move || {
                        app.update();
                    });

                GAME.push_scene(scene.clone(), JsValue::UNDEFINED);
            });
    }
}

#[derive(Resource, Debug, Default, Clone)]
struct SceneLoadedFlag(SharedObject<bool>);

impl SceneLoadedFlag {
    #[inline]
    pub fn set_loaded(&self) {
        *self.0.lock() = true;
    }

    #[inline]
    pub fn loaded(&self) -> bool {
        *self.0.lock()
    }
}

fn loading_scene_system(
    mut state: ResMut<NextState<SceneLoadState>>,
    scene_loaded_flag: Res<SceneLoadedFlag>,
) {
    if scene_loaded_flag.loaded() {
        state.set(SceneLoadState::Loaded);
    }
}

fn init_asset_server(mut state: ResMut<NextState<SceneLoadState>>, mut commands: Commands) {
    commands.insert_resource(AkashicAssetServer::default());
    state.set(SceneLoadState::Startup);
}

fn on_point_down_capture(scene: &impl PointDownCaptureHandler, world: &mut World) {
    let point_down_queue = world
        .resource::<AkashicEventQueue<ScenePointDown>>()
        .clone();
    let queue = world
        .resource::<AkashicEventQueue<PointDown>>()
        .clone();

    scene
        .on_point_down_capture()
        .add(move |e| {
            let point = e.point();
            if let Some(target) = e.target() {
                queue.push(PointDown {
                    entity_id: AkashicEntityId(target.id()),
                    point: Vec2::new(point.x(), point.y()),
                })
            } else {
                point_down_queue.push(ScenePointDown {
                    point: Vec2::new(point.x(), point.y()),
                })
            }
        });
}

fn on_point_up_capture(scene: &impl PointUpCaptureHandler, world: &mut World) {
    let point_down_queue = world
        .resource::<AkashicEventQueue<ScenePointUpEvent>>()
        .clone();
    scene
        .on_point_up_capture()
        .add(move |e| {
            let point = e.point();
            point_down_queue.push(ScenePointUpEvent {
                point: Vec2::new(point.x(), point.y()),
            })
        });
}

fn on_point_move_capture(scene: &impl PointMoveCaptureHandler, world: &mut World) {
    let point_down_queue = world
        .resource::<AkashicEventQueue<ScenePointMoveEvent>>()
        .clone();
    let queue = world
        .resource::<AkashicEventQueue<PointMoveEvent>>()
        .clone();
    scene
        .on_point_move_capture()
        .add(move |e| {
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
