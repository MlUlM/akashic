use akashic::font::dynamic::DynamicFontBuilder;
use akashic::font::font_family::FontFamily;
use akashic::game::GAME;
use akashic::prelude::{EntityObject2D, LabelBuilder};

fn main() {
    let font = DynamicFontBuilder::new(FontFamily::new("font-sans"), 32.).build();
    let label = LabelBuilder::new("Hello World!", font)
        .build();

    label.move_to_center();
    GAME
        .scene()
        .append(&label);
}