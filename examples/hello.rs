use std::panic;

use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_akashic::akashic::console_log;
use bevy_akashic::akashic::prelude::FilledRectBuilder;
use bevy_akashic::event::message::AddMessageEvent;
use bevy_akashic::plugin::asset::AkashicAssetServer;
use bevy_akashic::prelude::*;
use bevy_akashic::resource::game::GameInfo;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
struct Angel;

#[derive(Component, Debug)]
struct Shot;


#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct TestMessageEvent(String);


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(MyTimer(Timer::from_seconds(3., TimerMode::Repeating)))
        .add_message_event::<TestMessageEvent>()
        .add_plugins((
            TaskPoolPlugin::default(),
            FrameCountPlugin,
            TimePlugin,

        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)

        .run();
}

#[derive(Resource)]
struct MyTimer(Timer);


fn setup(mut commands: Commands, server: Res<AkashicAssetServer>, game_size: Res<GameInfo>) {
    console_log!("SETUP");

    let ground_size = 500.0;
    let ground_height = 10.0;
    let rect = FilledRectBuilder::new("blue", ground_size, ground_height)
        .x(game_size.width() / 2.)
        .y(game_size.height() / 2.)
        .build();
    commands.spawn((
        rect.into_bundle(),
        Collider::cuboid(ground_size, ground_height),
    ));
    let num = 8;
    let rad = 10.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            let x = i as f32 * shift - centerx + offset;
            let y = j as f32 * shift + centery + 30.0;

            let rect = FilledRectBuilder::new("red", 10., 10.)
                .x(x + game_size.width() / 2.)
                .y(y + game_size.height() / 2.)
                .build();
            commands
                .spawn(rect.into_bundle())
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(rad, rad));
        }

        offset += 0.05 * rad * (num as f32 - 1.0);
    }
}
