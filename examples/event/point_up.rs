use akashic::font::dynamic::DynamicFont;
use akashic::game::GAME;
use akashic::prelude::{CacheableEntityObject2D, EntityObject2D, FilledRectBuilder, LabelBuilder, PointUpHandler};
use akashic::trigger::{PointDeltaEventBase, PointEventBase};

fn main() {
    let background = FilledRectBuilder::new("#00000033", GAME.width(), GAME.height())
        .touchable(true)
        .build();

    let text = LabelBuilder::new("", DynamicFont::default()).build();
    background.append(text.clone());
    background.on_point_up().add(move |e| {
        let point = e.point();
        let start_delta = e.start_delta();
        text.set_text(format!("x = {} y = {}", point.x() + start_delta.x(), point.y() + start_delta.y()));
        text.invalidate();
    });

    GAME.scene().append(&background);
}