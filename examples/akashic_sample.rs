use akashic::asset::accessor::{AudioAssetAccessible, ImageAssetAccessible};
use akashic::game::GAME;
use akashic::object2d::Object2D;
use akashic::prelude::{EntityObject2D, PointDownCaptureHandler, SpriteBuilder, UpdateHandler};

fn main() {
    let scene = GAME.scene();
    let game_width = GAME.width();
    let game_height = GAME.height();

    let player_image = scene.asset().get_image("/assets/image/player.png");
    let player = SpriteBuilder::new(player_image)
        .x(GAME.half_width())
        .y(GAME.half_height())
        .build();
    let player_width = player.width();
    scene.append(&player);

    let p = player.clone();
    let fps = GAME.fps();
    scene.on_update().add(move || {
        p.set_y((game_height - p.height()) * 0.5 + (GAME.age() % (fps * 10.) / 4.).sin() * 10.);
        p.modified();
    });

    let shot_se = scene.asset().get_audio("/assets/audio/se/se");
    scene.on_point_down_capture().add(move |_| {
        shot_se.play();
        let scene = GAME.scene();
        let shot_image = scene.asset().get_image("/assets/image/shot.png");

        let shot = SpriteBuilder::new(shot_image).build();
        shot.move_to(
            player.x() + player_width,
            player.y(),
        );

        scene.append(&shot);
        shot.clone().on_update().add(move || {
            if shot.x() > game_width {
                shot.destroy();
            }

            shot.move_by(10., 0.);
            shot.modified();
        });
    });
}