use std::panic;

use bevy::app::{App, Startup, Update};
use bevy::core::{FrameCount, FrameCountPlugin};
use bevy::prelude::{Commands, Component, Event, Query, Res, ResMut, Resource, States, Timer, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::time::{Time, TimePlugin, TimerMode};

use bevy_akashic_engine::akashic::font::bitmap::{BitmapFont, BitmapFontParameterBuilder};
use bevy_akashic_engine::akashic::object2d::entity::cacheable::label::{Label, LabelParameterObjectBuilder, TextColor};
use bevy_akashic_engine::akashic::object2d::Object2D;
use bevy_akashic_engine::component::object2d::entity_size::AkashicEntitySize;
use bevy_akashic_engine::event::point_down::OnPointDown;
use bevy_akashic_engine::event::point_move::OnPointMove;
use bevy_akashic_engine::event::point_up::OnPointUp;
use bevy_akashic_engine::plugin::asset::AkashicAssetServer;
use bevy_akashic_engine::prelude::*;
use bevy_akashic_engine::prelude::scene::GameScene;
use bevy_akashic_engine::prelude::text::AkashicText;
use bevy_akashic_engine::resource::game::GameInfo;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Shot;


#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct TestMessageEvent(String);


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(MyTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .add_plugins((
            FrameCountPlugin,
            TimePlugin
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_hovering_system,
            spawn_player_system,
            update_label_system,
            point_down,
            point_move,
            point_up
        ))
        .run();
}

#[derive(Resource)]
struct MyTimer(Timer);


fn spawn_player_system(
    mut timer: ResMut<MyTimer>,
    time: Res<Time>,
    mut commands: Commands,
    server: Res<AkashicAssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let player_image_asset = server.image_by_id("player");
        let param = SpriteParameterObjectBuilder::new(player_image_asset)
            .local(true)
            .angle(90.)
            .x(100.)
            .touchable(true)
            .build();

        let player = Sprite::new(param);
        commands.spawn(player.into_bundle()).insert(Player);
    }
}


fn setup(mut commands: Commands, server: Res<AkashicAssetServer>, game_size: Res<GameInfo>) {
    console_log!("SETUP");

    let src = server.image_by_id("font");
    let font_glyphs = GAME.scene().asset().get_text_by_id("font_glyphs".to_string());

    let label = Label::new(LabelParameterObjectBuilder::new(
        "あかさたな",
        BitmapFont::new(BitmapFontParameterBuilder::new(src)
            .glyph_info(&font_glyphs.data())
            .build()
        ),
    )
        .build());

    commands.spawn(label.into_bundle());

    let player_image_asset = server.image_by_id("player");
    let param = SpriteParameterObjectBuilder::new(player_image_asset)
        .local(true)
        .touchable(true)
        .build();

    let player = Sprite::new(param);
    player.set_x((game_size.width() - player.width()) / 2.);
    player.set_y((game_size.height() - player.height()) / 2.);
    player.set_angle(45.);
    commands
        .spawn(player.into_bundle())
        .insert(Player);
}


fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Player>>,
    game_info: Res<GameInfo>,
    frames: Res<FrameCount>,
) {
    for (mut transform, size) in player.iter_mut() {
        transform.translation.y = (game_info.height() - size.height()) / 2. + ((frames.0 as f32) % (game_info.fps() * 10.) / 4.).sin() * 10.;
    }
}

fn update_label_system(
    mut player: Query<&mut AkashicText>,
    frames: Res<FrameCount>,
) {
    for mut text in player.iter_mut() {
        text.text = "テストアップデート".to_string();
        text.style.font_size = 30;
        let v = (frames.0 % 256) as u8;
        text.style.text_color = Some(TextColor::from_rgba(v, v, v, 1.));
    }
}


fn point_down(
    player: Query<&OnPointDown, With<GameScene>>
){
    for p in player.iter(){
        console_log!("down = {:?}", p.point());
    }
}


fn point_move(
    tests: Query<&OnPointMove, With<GameScene>>
){
    for p in tests.iter(){
        console_log!("move = {:?}", p.prev_delta());
    }
}


fn point_up(
    tests: Query<&OnPointUp, With<GameScene>>
){
    for p in tests.iter(){
        console_log!("up = {:?}", p.start_delta());
    }
}