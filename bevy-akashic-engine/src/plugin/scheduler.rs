use std::fmt::Debug;

use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{Deref, Resource, States, World};

use akashic_rs::object2d::entity::EntityObject2D;
use akashic_rs::prelude::Scene;
use akashic_rs::prelude::{PointDownCaptureHandler, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::trigger::point_move::PointMoveCaptureHandler;
use akashic_rs::trigger::point_up::PointUpCaptureHandler;
use akashic_rs::trigger::PointEventBase;

use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::point_down::{PointDown, ScenePointDown};
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::ScenePointUpEvent;
use crate::extensions::AsVec3;
use crate::prelude::point_move::ScenePointMoveEvent;



pub struct AkashicScheduleRunnerPlugin;


impl Plugin for AkashicScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_non_send_resource(GameScene(GAME.scene()))
            .set_runner(move |mut app| {
                let scene = app
                    .world
                    .non_send_resource::<GameScene>();

                // on_point_down_capture(&scene, &mut app.world);
                // on_point_up_capture(&scene, &mut app.world);
                // on_point_move_capture(&scene, &mut app.world);

                scene
                    .on_update()
                    .add(move || {
                        app.update();
                    });
            });
    }
}

#[derive(Debug, Deref)]
pub(crate) struct GameScene(pub(crate) Scene);


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
