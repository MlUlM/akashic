use std::panic;

use bevy::app::{App, Update};
use bevy::core::{FrameCount, FrameCountPlugin};
use bevy::prelude::{Commands, Component, Event, in_state, IntoSystemConfigs, OnEnter, Query, Res, States, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};

use bevy_akashic_engine::akashic::entity::label::{Label, LabelParameterObjectBuilder};
use bevy_akashic_engine::akashic::font::bitmap::{BitmapFont, BitmapFontParameterBuilder};
use bevy_akashic_engine::plugin::asset::AkashicAssetServer;
use bevy_akashic_engine::prelude::*;
use bevy_akashic_engine::prelude::entity_size::AkashicEntitySize;
use bevy_akashic_engine::prelude::SceneParameterObject;
use bevy_akashic_engine::prelude::src::IntoSrc;
use bevy_akashic_engine::resource::game::GameInfo;

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
        .add_systems(Update, (
            player_hovering_system
        ).run_if(in_state(SceneLoadState::Startup)))
        .run();
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


fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Player>>,
    game_info: Res<GameInfo>,
    frames: Res<FrameCount>,
) {
    let (mut transform, size) = player.single_mut();
    transform.translation.y = (game_info.height() - size.height()) / 2. + ((frames.0 as f32) % (game_info.fps() * 10.) / 4.).sin() * 10.;
}

