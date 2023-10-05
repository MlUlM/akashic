use akashic::font::dynamic::DynamicFontBuilder;
use akashic::font::font_family::FontFamily;
use akashic::game::GAME;
use akashic::prelude::{LabelBuilder, LoadHandler, SceneBuilder};

fn main() {
    let scene = SceneBuilder::default().build();

    scene.on_load().add(|_| {
        let font = DynamicFontBuilder::new(FontFamily::new("font-sans"), 32.).build();
        let label = LabelBuilder::new("Hello World!", font)
            .x(GAME.width() * 0.5)
            .y(GAME.height() * 0.5)
            .build();
        GAME
            .scene()
            .append(&label);
    });

    GAME.replace_scene(scene);
}