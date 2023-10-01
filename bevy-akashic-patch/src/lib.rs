use bevy::a11y::AccessibilityPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gilrs::GilrsPlugin;
use bevy::gizmos::GizmoPlugin;
use bevy::gltf::GltfPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::{ImagePlugin, TransformPlugin};
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::ui::UiPlugin;

use bevy_akashic::plugin::AkashicCorePlugins;
use bevy_akashic::prelude::AkashicScheduleRunnerPlugin;

use crate::asset::AkashicAssetIoPlugin;
use crate::input::AkashicInputPlugin;
use crate::window::AkashicWindowPlugin;

pub mod asset;
pub mod input;
mod window;


pub struct AkashicPatchDefaultPlugins;


impl PluginGroup for AkashicPatchDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogPlugin::default())
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(TimePlugin)
            .add(FrameCountPlugin)
            .add(InputPlugin)
            .add(bevy::window::WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .add(AkashicWindowPlugin)
            .add(AkashicAssetIoPlugin)
            .add(AssetPlugin::default())
            .add(TransformPlugin)
            .add(HierarchyPlugin)
            .add(DiagnosticsPlugin)
            .add(AccessibilityPlugin)
            .add(ScenePlugin)
            .add(RenderPlugin::default())
            .add(ImagePlugin::default())
            .add(CorePipelinePlugin)
            .add(SpritePlugin)
            .add(TextPlugin)
            .add(UiPlugin)
            .add(AkashicCorePlugins)
            .add(PbrPlugin::default())
            .add(GltfPlugin::default())
            .add(GizmoPlugin)
            .add(GilrsPlugin)
            .add(AkashicInputPlugin)
            .add(AkashicScheduleRunnerPlugin)
            .add(AudioPlugin::default())
    }
}
