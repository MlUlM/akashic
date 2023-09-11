use std::fmt::Debug;
use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Event, in_state, IntoSystemConfigs, NextState, Res, ResMut, Resource, States, World};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use akashic_rs::entity::E;
use akashic_rs::prelude::{Scene, SceneParameterObject};
use akashic_rs::prelude::{OnLoadHandler, PointDownCaptureHandler, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;
use akashic_rs::trigger::PointEventBase;

use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::message::add_akashic_message_event;
use crate::event::point_down::{PointDown, ScenePointDown};
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::ScenePointUpEvent;
use crate::extensions::AsVec3;
use crate::plugin::{SharedSceneParameter};
use crate::plugin::asset::AkashicAssetServer;
use crate::prelude::message::RegisterAkashicMessageFn;
use crate::prelude::point_move::ScenePointMoveEvent;
use crate::SharedObject;

#[derive(Resource)]
struct LoadedStateResource<S: States + Copy>(S);



pub struct AkashicSchedulerPlugin<S: States> {
    state_while_loading: S,
    state_loaded: S,
    scene_param: SharedSceneParameter,
    message_event_registers: Vec<RegisterAkashicMessageFn>
}

impl<S: States + Copy> AkashicSchedulerPlugin<S> {
    #[inline]
    pub fn new(state_while_loading: S, state_loaded: S) -> AkashicSchedulerPlugin<S>{
        Self{
            state_while_loading,
            state_loaded,
            scene_param: SharedSceneParameter::default(),
            message_event_registers: Vec::new()
        }
    }

    #[inline]
    pub fn with_scene_param(mut self, scene_param: SceneParameterObject) -> Self {
        self.scene_param = SharedSceneParameter::new(scene_param);
        self
    }


    #[inline]
    pub fn with_message_event<E>(mut self) -> Self
        where E: Event + Serialize + DeserializeOwned
    {
        self.message_event_registers.push(add_akashic_message_event::<E>());
        self
    }
}

impl<S: States + Copy> Plugin for AkashicSchedulerPlugin<S> {
    fn build(&self, app: &mut App) {
        let param = self.scene_param.clone();
        let message_event_registers = self.message_event_registers.clone();
        let scene_loaded_flag = SceneLoadedFlag::default();

        app
            .insert_resource(scene_loaded_flag.clone())
            .insert_resource(LoadedStateResource(self.state_loaded))
            .add_systems(
                PreUpdate,
                (loading_scene_system::<S>).run_if(in_state(self.state_while_loading)),
            )
            .set_runner(move |mut app| {
                let scene = Scene::new(param.param());

                on_point_down_capture(&scene, &mut app.world);
                on_point_up_capture(&scene, &mut app.world);
                on_point_move_capture(&scene, &mut app.world);

                for f in message_event_registers.iter() {
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

fn loading_scene_system<S: States + Copy>(
    mut commands: Commands,
    mut state: ResMut<NextState<S>>,
    scene_loaded_flag: Res<SceneLoadedFlag>,
    next_state: Res<LoadedStateResource<S>>
) {
    if scene_loaded_flag.loaded() {
        commands.insert_resource(AkashicAssetServer::default());
        state.set(next_state.0);
    }
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
