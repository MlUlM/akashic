use std::panic;

use bevy::a11y::AccessibilityPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;
use bevy::window::ExitCondition;
use bevy_akashic::akashic::prelude::FilledRectBuilder;

use bevy_akashic::prelude::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()

        .add_plugins((
            LogPlugin::default(),
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            FrameCountPlugin,
            TimePlugin,
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
            InputPlugin,
            AccessibilityPlugin,
            WindowPlugin{
                primary_window: Some(Window{
                    canvas: Some("bevy".to_string()),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_plugins((
            AssetPlugin::default(),
            ScenePlugin,
            RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends: None,
                    ..default()
                }
            },
            ImagePlugin::default(),
            CorePipelinePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    let rect = FilledRectBuilder::new("transparent", 300., 300.)
        .build();

    commands.spawn(rect.into_bundle())
        .insert(Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        });
}