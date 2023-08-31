use bevy::app::Update;
use bevy::prelude::{App, Commands, EventReader, OnEnter};

use bevy_akashic_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(AkashicPlugin)
        .add_systems(OnEnter(SceneLoadState::Loaded), setup)
        .add_systems(Update, read_point_down)
        .run();
}

fn setup(mut commands: Commands) {
    commands.append(FilledRect::new(FilledRectParameter {
        scene: GAME.scene(),
        css_color: "#ff0000".to_string(),
        width: 100.,
        height: 100.,
        touchable: true,
    }));
}


fn read_point_down(
    mut er: EventReader<PointDown>
){
    for e in er.iter(){
        console_log!("{:?}", e);
    }
}