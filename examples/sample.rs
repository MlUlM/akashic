use std::panic;

use bevy::prelude::*;
use bevy::time::TimePlugin;

use bevy_akashic::akashic::prelude::FilledRectBuilder;
use bevy_akashic::prelude::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins((
            TaskPoolPlugin::default(),
            FrameCountPlugin,
            TimePlugin
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands
) {
    let rect = FilledRectBuilder::new("red", 300., 300.)
        .build();

    commands.spawn(rect.into_bundle())
        .with_children(|parent| {
            parent.spawn(FilledRectBuilder::new("blue", 100., 100.)
                .build()
                .into_bundle()
            );
        });
}