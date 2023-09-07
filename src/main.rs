use std::panic;

use bevy::app::{App, PluginGroup, PluginGroupBuilder, Update};
use bevy::core::{FrameCount, FrameCountPlugin, TypeRegistrationPlugin};
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::log::LogPlugin;
use bevy::math::Quat;
use bevy::prelude::{Commands, Component, EventReader, in_state, IntoSystemConfigs, OnEnter, Query, Res, Time, Transform, TransformPlugin, With};
use bevy::time::TimePlugin;

use bevy_akashic_engine::prelude::*;
use bevy_akashic_engine::prelude::entity_size::AkashicEntitySize;
use bevy_akashic_engine::prelude::game::GameInfo;
use bevy_akashic_engine::prelude::point_down::ScenePointDown;
use bevy_akashic_engine::prelude::SceneParameterObject;
use bevy_akashic_engine::prelude::src::IntoSrc;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Shot;

pub struct AkashicDefaultPlugin;

impl PluginGroup for AkashicDefaultPlugin {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(LogPlugin::default())
            .add(TypeRegistrationPlugin)
            .add(FrameCountPlugin)
            .add(TimePlugin)
            .add(TransformPlugin)
            .add(HierarchyPlugin)
            .add(DiagnosticsPlugin)
        //   .add(AccessibilityPlugin)
        // .add(InputPlugin)
        //        .add(TaskPoolPlugin::default())
        //     .add(AssetPlugin::default())
        // .add(WindowPlugin::default())
        // .add(CorePipelinePlugin)
        // .add(SpritePlugin)
        // .add(RenderPlugin::default())
        // .add(ImagePlugin::default())
        // .add(GizmoPlugin)
        // .add(PbrPlugin::default())
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(FrameCountPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(AkashicPlugin::new(SceneParameterObject::builder(GAME.clone())
            .asset_ids(vec!["player", "shot", "se"])
            .build()
        ))
        .add_systems(OnEnter(SceneLoadState::Startup), setup)
        .add_systems(Update, (
            player_hovering_system,
            read_scene_point_down_event,
            shot_move_system
        ).run_if(in_state(SceneLoadState::Startup)))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AkashicAssetServer>,
    game_size: Res<GameInfo>,
) {
    let player_image_asset = server.image_by_id("player").into_src();
    let a = SpriteParameterObjectBuilder::default()
        .src(player_image_asset)
        .build()
        .unwrap();
    console_log!("{a:?}");
    let player = Sprite::new(a);

    player.set_x((game_size.width - player.width()) / 2.);
    player.set_y((game_size.height - player.height()) / 2.);

    commands
        .append(player)
        .insert(Player);
}


fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Player>>,
    game_info: Res<GameInfo>,
    frames: Res<FrameCount>,
) {
    let (mut transform, size) = player.single_mut();
    transform.translation.y = (game_info.height - size.height()) / 2. + ((frames.0 as f32) % (game_info.fps * 10.) / 4.).sin() * 10.;
}


fn read_scene_point_down_event(
    mut commands: Commands,
    mut er: EventReader<ScenePointDown>,
    server: Res<AkashicAssetServer>,
    player: Query<(&Transform, &AkashicEntitySize), With<Player>>,
) {
    for _ in er.iter() {
        let (player_transform, player_size) = player.single();
        let player_pos = player_transform.translation;
        let shot_image_asset = server.image_by_id("shot").into_src();
        // 弾の初期座標を、プレイヤーの少し右に設定します
        let shot = Sprite::new(SpriteParameterObjectBuilder::default()
            .src(shot_image_asset)
            .x(player_pos.x + player_size.width())
            .y(player_pos.y)
            .build()
            .unwrap()
        );

        commands.append(shot).insert(Shot);
        commands.play_audio(server.audio_by_id("se"));
    }
}


fn shot_move_system(
    mut commands: Commands,
    mut shots: Query<(bevy::prelude::Entity, &mut Transform), With<Shot>>,
    game_info: Res<GameInfo>,
) {
    for (entity, mut shot) in shots.iter_mut() {
        if game_info.width < shot.translation.x {
            commands.entity(entity).despawn();
        }

        shot.translation.x += 10.;
    }
}