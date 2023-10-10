use akashic::asset::accessor::AudioAssetAccessible;
use akashic::game::GAME;

fn main() {
    let audio = GAME.scene().asset().get_audio("/assets/audio/se/se");
    audio.play();
}