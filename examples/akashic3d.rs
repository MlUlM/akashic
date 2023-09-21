use std::panic;

use bevy::a11y::AccessibilityPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;

use bevy_akashic::plugin::akashic_3d::Akashic3DPlugin;
use bevy_akashic::prelude::*;

#[derive(Component)]
struct Cube;

fn main() {
    // env_logger::init();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins((
            LogPlugin::default(),
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            FrameCountPlugin,
            WindowPlugin {
                primary_window: Some(Window {
                    transparent: true,
                    ..default()
                }),
                ..default()
            },
            TimePlugin,
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
            InputPlugin,
            AccessibilityPlugin,
        ))
        .add_plugins((
            AssetPlugin::default(),
            ScenePlugin,
            AkashicMinimumPlugins,
            Akashic3DPlugin,
        ))

        .run();
}


