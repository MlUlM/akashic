use akashic::font::dynamic::DynamicFontBuilder;
use akashic::font::font_family::FontFamily;
use akashic::game::GAME;
use akashic::prelude::{EntityObject2D, LabelBuilder, LoadHandler, PointDownHandler, SceneBuilder};

fn main() {
    let font = DynamicFontBuilder::new(FontFamily::sans_serif(), 50.).build();
    GAME.scene().append(&LabelBuilder::new("Scene 1", font.clone())
        .anchor_x(0.)
        .anchor_y(0.)
        .text_color("blue")
        .build()
    );

    let next_scene_button = LabelBuilder::new(
        "Click here",
        font,
    )
        .touchable(true)
        .build();
    next_scene_button
        .on_point_down()
        .add(|_| { next_scene(); });
    next_scene_button.move_to_center();
    GAME.scene().append(&next_scene_button);
}


fn next_scene() {
    let scene = SceneBuilder::default().build();
    scene.on_load().add(|scene| {
        let font = DynamicFontBuilder::new(FontFamily::sans_serif(), 50.).build();
        let label = LabelBuilder::new(
            "Scene 2",
            font,
        )
            .anchor_x(0.)
            .anchor_y(0.)
            .text_color("blue")
            .build();

        scene.append(&label);
    });
    GAME.push_scene(scene);
}