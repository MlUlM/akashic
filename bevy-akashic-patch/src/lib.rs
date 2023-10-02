pub mod asset;
pub mod input;
pub mod window;
pub use plugin::AkashicPatchDefaultPlugins;

pub mod bevy {
    pub use bevy::*;
}


mod plugin {
    use bevy_akashic::plugin::AkashicCorePlugins;
    use bevy_akashic::prelude::AkashicScheduleRunnerPlugin;
    use crate::asset::AkashicAssetIoPlugin;
    use crate::bevy::a11y::AccessibilityPlugin;
    use crate::bevy::app::{PluginGroup, PluginGroupBuilder};
    use crate::bevy::asset::AssetPlugin;
    use crate::bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
    use crate::bevy::diagnostic::DiagnosticsPlugin;
    use crate::bevy::hierarchy::HierarchyPlugin;
    use crate::bevy::input::InputPlugin;
    use crate::bevy::log::LogPlugin;
    use crate::bevy::prelude::TransformPlugin;
    use crate::bevy::time::TimePlugin;
    use crate::input::AkashicInputPlugin;
    use crate::window::AkashicWindowPlugin;

    pub struct AkashicPatchDefaultPlugins;


    impl PluginGroup for AkashicPatchDefaultPlugins {
        fn build(self) -> PluginGroupBuilder {
            let mut group = PluginGroupBuilder::start::<Self>()
                .add(LogPlugin::default())
                .add(TaskPoolPlugin::default())
                .add(TypeRegistrationPlugin::default())
                .add(FrameCountPlugin)
                .add(TimePlugin)
                .add(TransformPlugin)
                .add(HierarchyPlugin)
                .add(DiagnosticsPlugin)
                .add(InputPlugin)
                .add(crate::bevy::window::WindowPlugin::default())
                .add(AccessibilityPlugin)
                .add(AkashicWindowPlugin);

            #[cfg(feature = "asset")]
            {
                group = group
                    .add(AkashicAssetIoPlugin)
                    .add(AssetPlugin::default());
            }

            #[cfg(feature = "debug_asset_server")]
            {
                group = group
                    .add(crate::bevy::asset::debug_asset_server::DebugAssetServerPlugin);
            }


            #[cfg(feature = "bevy_scene")]
            {
                group = group.add(crate::bevy::scene::ScenePlugin);
            }


            #[cfg(feature = "bevy_render")]
            {
                group = group
                    .add(crate::bevy::render::RenderPlugin::default())
                    .add(crate::bevy::render::texture::ImagePlugin::default());
            }
            #[cfg(feature = "bevy_core_pipeline")]
            {
                group = group.add(crate::bevy::core_pipeline::CorePipelinePlugin);
            }

            #[cfg(feature = "bevy_sprite")]
            {
                group = group.add(crate::bevy::sprite::SpritePlugin);
            }

            #[cfg(feature = "bevy_text")]
            {
                group = group.add(crate::bevy::text::TextPlugin);
            }

            #[cfg(feature = "bevy_ui")]
            {
                group = group.add(crate::bevy::ui::UiPlugin);
            }

            group = group.add(AkashicCorePlugins);

            #[cfg(feature = "bevy_pbr")]
            {
                group = group.add(crate::bevy::pbr::PbrPlugin::default());
            }

            #[cfg(feature = "bevy_gltf")]
            {
                group = group.add(crate::bevy::gltf::GltfPlugin::default());
            }
            #[cfg(feature = "bevy_gizmos")]
            {
                group = group.add(crate::bevy::gizmos::GizmoPlugin);
            }

            #[cfg(feature = "bevy_gilrs")]
            {
                group = group.add(crate::bevy::gilrs::GilrsPlugin);
            }


            #[cfg(feature = "bevy_audio")]
            {
                group = group.add(crate::bevy::audio::AudioPlugin::default());
            }

            group
                .add(AkashicInputPlugin)
                .add(AkashicScheduleRunnerPlugin)
        }
    }
}

