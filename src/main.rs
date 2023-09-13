use std::panic;

use bevy::app::{App, Update};
use bevy::core::{FrameCount, FrameCountPlugin};
use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, OnEnter, Query, Res, States, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

use bevy_akashic_engine::akashic::entity::label::{Label, LabelParameterObjectBuilder};
use bevy_akashic_engine::akashic::font::bitmap::{BitmapFont, BitmapFontParameterBuilder};
use bevy_akashic_engine::akashic::font::dynamic::{DynamicFont, DynamicFontParameterObjectBuilder};
use bevy_akashic_engine::akashic::font::font_family::FontFamily;
use bevy_akashic_engine::event::message::AkashicRaiseEvent;
use bevy_akashic_engine::plugin::asset::AkashicAssetServer;
use bevy_akashic_engine::prelude::*;
use bevy_akashic_engine::prelude::entity_size::AkashicEntitySize;
use bevy_akashic_engine::prelude::point_down::ScenePointDown;
use bevy_akashic_engine::prelude::SceneParameterObject;
use bevy_akashic_engine::prelude::src::IntoSrc;
use bevy_akashic_engine::resource::game::GameInfo;
use bevy_akashic_engine::resource::join::{JoinedAsListener, JoinedAsStreamer};

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Shot;


#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct TestMessageEvent {
    message: String,
}

#[derive(States, Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
enum SceneLoadState {
    #[default]
    Loading,
    Startup,
}


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let scene_param = SceneParameterObject::builder(GAME.clone())
        .asset_ids(vec!["player", "shot", "se", "font", "font_glyphs"])
        .build();

    App::new()
        .add_state::<SceneLoadState>()
        .add_plugins(FrameCountPlugin)
        .add_plugins(AkashicMinimumPlugins)
        .add_plugins(AkashicSchedulerPlugin::new(SceneLoadState::Loading, SceneLoadState::Startup)
            .with_scene_param(scene_param)
        )
        .add_systems(OnEnter(SceneLoadState::Startup), setup)
        .add_systems(Update, player_hovering_system.run_if(in_state(SceneLoadState::Startup)))
        .run();
}

fn set(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: bevy::prelude::Sprite {
            custom_size: Some(Vec2::new(30., 30.)),
            color: Color::RED,
            ..default()
        },
        ..default()
    });
}

fn setup(mut commands: Commands, server: Res<AkashicAssetServer>, game_size: Res<GameInfo>) {
    console_log!("SETUP");

    let src = server.image_by_id("font");
    let font_glyphs = GAME.scene().asset().get_text_by_id("font_glyphs".to_string());

    let label = Label::new(LabelParameterObjectBuilder::new(
        "あかさたな",
        BitmapFont::new(BitmapFontParameterBuilder::new(src.into_src())
                .glyph_info(&font_glyphs.data())
                .build()
        ),
    )
        .max_width(300.)
        .font_size(80.)
        .build());

    commands.append(label);

    let player_image_asset = server
        .image_by_id("player")
        .into_src();
    let param = SpriteParameterObjectBuilder::new(player_image_asset)
        .local(true)
        .touchable(true)
        .build();

    let player = Sprite::new(param);
    player.set_x((game_size.width() - player.width()) / 2.);
    player.set_y((game_size.height() - player.height()) / 2.);
    player.set_angle(45.);
    commands
        .append(player)
        .insert(Player);
}

fn setup_streamer(mut commands: Commands, joined: Res<JoinedAsStreamer>) {
    let font = DynamicFont::new(
        DynamicFontParameterObjectBuilder::new(FontFamily::new("sans-serif"), 30.)
            .font_color("blue")
            .build(),
    );

    let text = format!("あなたは放送主です。 ID = {}", joined.player_id_as_str());
    commands.append(Label::new(
        LabelParameterObjectBuilder::new(text, font)
            .local(true)
            .build(),
    ));
}

fn setup_listener(mut commands: Commands, joined: Res<JoinedAsListener>) {
    let font = DynamicFont::new(
        DynamicFontParameterObjectBuilder::new(FontFamily::new("sans-serif"), 30.)
            .font_color("blue")
            .build(),
    );

    let text = format!("あなたは参加者です。 ID = {}", joined.player_id_as_str());
    commands.append(Label::new(LabelParameterObjectBuilder::new(text, font)
        .local(true)
        .build()
    ));
}

fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Player>>,
    game_info: Res<GameInfo>,
    frames: Res<FrameCount>,
) {
    let (mut transform, size) = player.single_mut();
    transform.translation.y = (game_info.height() - size.height()) / 2. + ((frames.0 as f32) % (game_info.fps() * 10.) / 4.).sin() * 10.;
}

fn read_scene_point_down_event(
    mut commands: Commands,
    mut ew: EventWriter<AkashicRaiseEvent<TestMessageEvent>>,
    mut er: EventReader<ScenePointDown>,
    server: Res<AkashicAssetServer>,
    player: Query<(&Transform, &AkashicEntitySize), With<Player>>,
) {
    for _ in er.iter() {
        ew.send(AkashicRaiseEvent {
            data: TestMessageEvent {
                message: "TEST HELLO !!".to_string(),
            },
            ..default()
        });

        let (player_transform, player_size) = player.single();
        let player_pos = player_transform.translation;
        let shot_image_asset = server
            .image_by_id("shot")
            .into_src();
        // 弾の初期座標を、プレイヤーの少し右に設定します
        let shot = Sprite::new(
            SpriteParameterObjectBuilder::new(shot_image_asset)
                .x(player_pos.x + player_size.width())
                .y(player_pos.y)
                .build(),
        );

        commands
            .append(shot)
            .insert(Shot);
        commands.play_audio(server.audio_by_id("se"));
    }
}

fn shot_move_system(
    mut commands: Commands,
    mut shots: Query<(bevy::prelude::Entity, &mut Transform), With<Shot>>,
    game_info: Res<GameInfo>,
) {
    for (entity, mut shot) in shots.iter_mut() {
        if game_info.width() < shot.translation.x {
            commands
                .entity(entity)
                .despawn();
        }

        shot.translation.x += 10.;
    }
}

fn point_up_event_system(mut er: EventReader<bevy_akashic_engine::event::point_up::PointUpEvent>) {
    for e in er.iter() {
        console_log!("{e:?}");
    }
}

fn read_raise_event_system(mut er: EventReader<TestMessageEvent>) {
    for e in er.iter() {
        console_log!("{e:?}");
    }
}
