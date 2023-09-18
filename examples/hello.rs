use std::panic;

use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_rapier2d::prelude::*;

use bevy_akashic::akashic::prelude::FilledRectBuilder;
use bevy_akashic::plugin::akashic_3d::{Akashic3DPlugin, is_node};
use bevy_akashic::prelude::*;
use bevy_akashic::resource::game::GameInfo;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins((
            TaskPoolPlugin::default(),
            FrameCountPlugin,
            TimePlugin
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands, game_info: Res<GameInfo>) {
    commands.spawn(FilledRectBuilder::new("gray", game_info.width(), game_info.height())
        .build()
        .into_bundle()
    );


    let ground_size = 500.0;
    let ground_height = 10.0;
    let rect = FilledRectBuilder::new("red", ground_size * 2., ground_height * 2.).build();

    commands.spawn((
        rect.into_bundle(),
        Collider::cuboid(ground_size, ground_height),
    ));

    let num = 30;
    let rad = 10.0;

    let shift = rad * 2.0 + rad;
    let center_x = shift * (num / 2) as f32;
    let center_y = shift / 2.0;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            let x = i as f32 * shift - center_x + offset + 300.;
            let y = j as f32 * shift + center_y + 30.0;

            let rect = FilledRectBuilder::new("blue", rad * 2., rad * 2.)
                .x(x)
                .y(y)
                .build();
            commands
                .spawn(rect.into_bundle())
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(rad, rad));
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}


