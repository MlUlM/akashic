use std::panic;

use bevy::app::{App, PluginGroup, PluginGroupBuilder, Update};
use bevy::core::{FrameCount, FrameCountPlugin, TypeRegistrationPlugin};
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::{Commands, Component, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, OnEnter, Query, Res, Transform, TransformPlugin, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::time::TimePlugin;
use bevy::utils::default;

use bevy_akashic_engine::akashic::entity::label::{Label, LabelParameterObjectBuilder};
use bevy_akashic_engine::akashic::font::dynamic::{DynamicFont, DynamicFontParameterObjectBuilder};
use bevy_akashic_engine::akashic::font::font_family::FontFamily;
use bevy_akashic_engine::event::message::{AkashicRaiseEvent};
use bevy_akashic_engine::event::point_down::PointDown;
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


#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct TestMessageEvent {
    message: String,
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let scene_param = SceneParameterObject::builder(GAME.clone())
        .asset_ids(vec!["player", "shot", "se"])
        .build();
    App::new()
        .add_plugins(FrameCountPlugin)

        .add_plugins(AkashicPlugin::new(scene_param).add_message_event::<TestMessageEvent>())
        .add_systems(OnEnter(SceneLoadState::Startup), (
            setup,
            setup_text
        ))
        .add_systems(Update, (
            read_scene_point_down_event,
            shot_move_system,
            point_up_event_system,
            player_hovering_system,
            read_raise_event_system
        ).run_if(in_state(SceneLoadState::Startup)))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AkashicAssetServer>,
    game_size: Res<GameInfo>,
) {
    let player_image_asset = server.image_by_id("player").into_src();
    let param = SpriteParameterObjectBuilder::new(player_image_asset)
        .local(true)
        .touchable(true)
        .build();

    let player = Sprite::new(param);
    player.set_x((game_size.width - player.width()) / 2.);
    player.set_y((game_size.height - player.height()) / 2.);
    player.set_angle(45.);
    commands.append(player).insert(Player);
}

fn setup_text(
    mut commands: Commands,
) {
    let font = DynamicFont::new(DynamicFontParameterObjectBuilder::new(FontFamily::new("sans-serif"), 30.)
        .font_color("blue")
        .build()
    );

    commands.append(Label::new(LabelParameterObjectBuilder::new("Hello World", font).build()));
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
    mut ew: EventWriter<AkashicRaiseEvent<TestMessageEvent>>,
    mut er: EventReader<PointDown>,
    server: Res<AkashicAssetServer>,
    player: Query<(&Transform, &AkashicEntitySize), With<Player>>,
) {
    for _ in er.iter() {
        ew.send(AkashicRaiseEvent {
            data: TestMessageEvent {
                message: "TEST HELLO !!".to_string()
            },
            ..default()
        });

        let (player_transform, player_size) = player.single();
        let player_pos = player_transform.translation;
        let shot_image_asset = server.image_by_id("shot").into_src();
        // 弾の初期座標を、プレイヤーの少し右に設定します
        let shot = Sprite::new(SpriteParameterObjectBuilder::new(shot_image_asset)
            .x(player_pos.x + player_size.width())
            .y(player_pos.y)
            .build()
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


fn point_up_event_system(
    mut er: EventReader<bevy_akashic_engine::event::point_up::PointUpEvent>
) {
    for e in er.iter() {
        console_log!("{e:?}");
    }
}


fn read_raise_event_system(
    mut er: EventReader<TestMessageEvent>
) {
    for e in er.iter() {
        console_log!("{e:?}");
    }
}