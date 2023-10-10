use akashic::asset::accessor::ImageAssetAccessible;
use akashic::game::GAME;
use akashic::prelude::SpriteBuilder;


fn main() {
    let scene = GAME.scene();
    let image = scene.asset().get_image("/assets/image/player.png");
    let sprite = SpriteBuilder::new(image)
        .x(GAME.half_width())
        .y(GAME.half_height())
        .touchable(false)
        .anchor_x(0.)
        .anchor_y(0.)
        .angle(0.)
        .local(false)
        .scene(GAME.scene())
        .opacity(0.8)
        .scale_x(1.5)
        .scale_y(1.5)
        .build();

    GAME
        .scene()
        .append(&sprite);
}