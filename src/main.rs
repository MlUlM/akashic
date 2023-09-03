use bevy::app::Update;
use bevy::prelude::{App, Commands, Component, EventReader, OnEnter, Query, Res, Transform, With};

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


fn main() {
    App::new()
        .add_plugins(AkashicPlugin::new(SceneParameterObject::builder(GAME.clone())
            .asset_ids(vec!["player", "shot", "se"])
            .build()
        ))
        .add_systems(OnEnter(SceneLoadState::Loaded), setup)
        .add_systems(Update, (
            player_hovering_system,
            read_scene_point_down_event,
            shot_move_system
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AkashicAssetServer>,
    game_size: Res<GameInfo>,
) {
    let player_image_asset = server.get_image_by_id("player").into_src();
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
    let (mut transform, size) = player.single_mut();
    transform.translation.y = (game_info.height() - size.height()) / 2. + (game_info.age() % (game_info.fps() * 10.) / 4.).sin() * 10.;
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
        let shot_image_asset = server.get_image_by_id("shot").into_src();
        // 弾の初期座標を、プレイヤーの少し右に設定します
        let shot = Sprite::new(SpriteParameterObject::builder(GAME.scene(), shot_image_asset)
            .x(player_pos.x + player_size.width())
            .y(player_pos.y)
            .build()
        );

        commands.append(shot).insert(Shot);
        commands.play_audio(server.get_audio_by_id("se"));
    }
}


fn shot_move_system(
    mut commands: Commands,
    mut shots: Query<(bevy::prelude::Entity, &mut Transform), With<Shot>>,
    game_info: Res<GameInfo>,
) {
    for (entity, mut shot) in shots.iter_mut() {
        if game_info.width() < shot.translation.x {
            commands.entity(entity).despawn();
        }

        shot.translation.x += 10.;
    }
}