use akashic::game::GAME;
use akashic::prelude::FilledRectBuilder;

fn main() {
    let css_color = "blue";
    let width = GAME.width();
    let height = GAME.height();

    let rect = FilledRectBuilder::new(css_color, width, height)
        .x(0.)
        .y(0.)
        .touchable(false)
        .anchor_x(0.)
        .anchor_y(0.)
        .angle(0.)
        .local(false)
        .scene(GAME.scene())
        .opacity(0.1)
        .scale_x(0.5)
        .scale_y(0.5)
        .build();

    GAME
        .scene()
        .append(&rect);
}