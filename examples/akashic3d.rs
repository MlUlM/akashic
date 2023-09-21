use std::panic;

use bevy::a11y::AccessibilityPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::renderer::{RenderAdapter, RenderDevice};
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;

use bevy_akashic::akashic::prelude::SpriteBuilder;
use bevy_akashic::plugin::akashic_3d::{Akashic3DPlugin, AkashicSurface, WgpuInstance};
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
        .add_systems(Startup, setup)
        .add_systems(Update, move_cube)
        .run();
}


fn setup(
    mut commands: Commands,
    akashic_surface: NonSend<AkashicSurface>,
    instance: Res<WgpuInstance>,
    adapter: Res<RenderAdapter>,
    device: Res<RenderDevice>
) {
    commands.spawn(SpriteBuilder::new(akashic_surface.0.clone())
        .width(200.)
        .height(200.)
        .build()
        .into_bundle()
    )
        .insert(Cube)
        .insert(instance.create_surface(Vec2::new(200., 200.), &akashic_surface, &adapter, &device));
}


fn move_cube(
    mut cube: Query<&mut Transform, With<Cube>>
) {
    for mut transform in cube.iter_mut() {
        transform.translation += Vec3::X;
    }
}