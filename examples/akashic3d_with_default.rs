use std::panic;

use bevy::a11y::AccessibilityPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::render_phase::RenderPhase;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;

use bevy_akashic::akashic::prelude::SpriteBuilder;
use bevy_akashic::plugin::akashic_3d::AkashicSurface;
use bevy_akashic::plugin::akashic_3d2::{Akashic3D2Plugin, PhaseAkashic};
use bevy_akashic::plugin::winit::AkashicWinitPlugin;
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
            AkashicWinitPlugin,
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
            RenderPlugin::default(),
            ImagePlugin::default(),
            CorePipelinePlugin,
            AkashicMinimumPlugins,
            Akashic3D2Plugin,
            // PbrPlugin::default()
        ))
        .add_systems(Startup, (
            setup2,
            setup2
            ))
        .add_systems(Update, move_cube)
        .run();
}


fn setup2(
    mut commands: Commands,
    akashic_surface: NonSend<AkashicSurface>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBuilder::new(akashic_surface.0.clone())
        .width(100.)
        .height(100.)
        .build()
        .into_bundle()
    )
        .insert(Cube)
        .insert(RenderPhase::<PhaseAkashic>::default());
}


fn move_cube(
    mut cube: Query<&mut Transform, With<Cube>>
) {
    for mut transform in cube.iter_mut() {
        transform.translation += Vec3::X;
    }
}
//
// #[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
// #[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
// struct CustomMaterial {}
//
// impl Material for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/animate_shader.wgsl".into()
//     }
// }
