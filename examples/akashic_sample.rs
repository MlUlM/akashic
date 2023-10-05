use akashic::asset::accessor::{AudioAssetAccessible, ImageAssetAccessible};
use akashic::game::GAME;
use akashic::object2d::Object2D;
use akashic::prelude::{EntityObject2D, LoadHandler, PointDownCaptureHandler, SceneBuilder, SpriteBuilder, UpdateHandler};

fn main() {
    let scene = SceneBuilder::default()
        .assets_paths(vec!["/assets/**/*"])
        .build();

    scene.on_load().add(|_| {
        let game_width = GAME.width();
        let game_height = GAME.height();

        let scene = GAME.scene();

        let player_image = scene.asset().get_image("/assets/image/player.png");
        let player = SpriteBuilder::new(player_image).build();
        let player_width = player.width();
        let player_height = player.height();

        player.move_to(
            (game_width - player_width) * 0.5,
            (game_height - player_height) * 0.5,
        );

        scene.append(&player);

        let p = player.clone();
        let fps = GAME.fps();
        scene.on_update().add(move || {
            p.set_y((game_height - p.height()) * 0.5 + (GAME.age() % (fps * 10.) / 4.).sin() * 10.);
            p.modified();
        });

        let shot_se = scene.asset().get_audio("/assets/audio/se");
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
    });

    GAME.replace_scene(scene);
}