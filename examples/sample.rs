use std::panic;

use bevy::a11y::AccessibilityPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::view::WindowSurfaces;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;

use bevy_akashic::akashic::console_log;
use bevy_akashic::prelude::*;

fn main() {
    env_logger::init();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()

        .add_plugins((
            LogPlugin::default(),
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            FrameCountPlugin,
            WindowPlugin::default(),
            TimePlugin,
            AkashicMinimumPlugins,
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
            InputPlugin,
            AccessibilityPlugin,
        ))
        .add_plugins((
            AssetPlugin::default(),
            ScenePlugin,
            RenderPlugin{
                wgpu_settings: WgpuSettings{
                    backends: Some(Backends::GL),
                    ..default()
                }
            },
            ImagePlugin::default(),
            CorePipelinePlugin,
            PbrPlugin::default(),
            MaterialPlugin::<CustomMaterial>::default()
        ))
        .add_systems(Startup, setup2)
        // .add_systems(Update, read)
        .run();
}


fn setup2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    console_log!("DADA");
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn read(
    win: Res<WindowSurfaces>
) {
    console_log!("add {}", win.is_added());
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
