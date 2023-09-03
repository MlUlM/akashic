use bevy::app::Update;
use bevy::prelude::{App, Commands, Component, EventReader, OnEnter, Query, Res, Transform, With};

use bevy_akashic_engine::prelude::*;
use bevy_akashic_engine::prelude::append::AkashicCommandEx;
use bevy_akashic_engine::prelude::entity_size::AkashicEntitySize;
use bevy_akashic_engine::prelude::game::GameInfo;
use bevy_akashic_engine::prelude::SceneParameterObject;
use bevy_akashic_engine::prelude::point_down::{PointDown, ScenePointDown};
use bevy_akashic_engine::prelude::src::IntoSrc;

#[derive(Component, Debug)]
struct Player;


fn main() {
    App::new()
        .add_plugins(AkashicPlugin::new(SceneParameterObject::builder(GAME.clone())
            .asset_ids(vec!["player", "shot", "se"])
            .build()
        ))
        .add_systems(OnEnter(SceneLoadState::Loaded), setup)

        .add_systems(Update, (
            player_hovering_system,
            read_point_down,
            read_scene_point_down_event
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AkashicAssetServer>,
    game_size: Res<GameInfo>,
) {
    console_log!("setup");


    let player_image_asset = server.get_image_by_id("player").into_src();
    let shot_image_asset = server.get_image_by_id("shot");
    // let se_audio_asset = server.asset.get_audio_by_id("se");

    let player = Sprite::new(SpriteParameterObject::builder(GAME.scene().clone(), player_image_asset)
        .build()
    );

    player.set_x((game_size.width() - player.width()) / 2.);
    player.set_y((game_size.height() - player.height()) / 2.);

    commands
        .append(player)
        .insert(Player);
}

fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Player>>,
    game_info: Res<GameInfo>,
) {
    console_log!("UPDATE");
    let (mut transform, size) = player.single_mut();
    transform.translation.y = (game_info.height() - size.height()) / 2. + (game_info.age() % (game_info.fps() * 10.) / 4.).sin() * 10.;
}

fn read_point_down(
    mut er: EventReader<PointDown>,
    rects: Query<&AkashicEntityId>,
) {
    for event in er.iter() {
        let id = rects.iter().find(|r| **r == event.entity_id);
        console_log!("{:?}", id);
    }
}


fn read_scene_point_down_event(
    mut er: EventReader<ScenePointDown>
) {
    for e in er.iter() {
        console_log!("on point down scene! {:?}", e.point);
    }
}