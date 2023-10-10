use akashic::font::dynamic::DynamicFont;
use akashic::game::GAME;
use akashic::prelude::{CacheableEntityObject2D, EntityObject2D, FilledRectBuilder, LabelBuilder, PointDownHandler};
use akashic::trigger::PointEventBase;

fn main() {
    let background = FilledRectBuilder::new("#00000033", GAME.width(), GAME.height())
        .touchable(true)
        .build();

    let text = LabelBuilder::new("", DynamicFont::default()).build();
    background.append(text.clone());
    background.on_point_down().add(move |e| {
        let point = e.point();
        text.set_text(format!("x = {} y = {}", point.x(), point.y()));
        text.invalidate();
    });

    GAME.scene().append(&background);
}