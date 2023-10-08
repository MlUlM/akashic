use akashic::asset::accessor::AudioAssetAccessible;
use akashic::game::GAME;
use akashic::prelude::{LoadHandler, SceneBuilder};

fn main() {
    let scene = SceneBuilder::default().build();
    scene.on_load().add(|_| {
        let audio = GAME.scene().asset().get_audio("/assets/audio/se/se");
        audio.play();
    });

    GAME.replace_scene(scene);
}