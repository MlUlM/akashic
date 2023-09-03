use bevy::app::Update;
use bevy::prelude::{App, Commands, EventReader, OnEnter, Query, Transform, With};

use bevy_akashic_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(AkashicPlugin::new(SceneParameterObject::builder(GAME.clone())
            .asset_ids(vec!["player"])
            .build()
        ))
        .add_systems(OnEnter(SceneLoadState::Loaded), setup)
        .add_systems(Update, move_system)
        .add_systems(Update, (
            read_point_down,
            read_scene_point_down_event
        ))
        .run();
}

fn setup(mut commands: Commands) {
    console_log!("setup");
    commands.append(FilledRect::new(FilledRectParameter {
        scene: GAME.scene(),
        css_color: "#ff0000".to_string(),
        width: 100.,
        height: 100.,
        touchable: true,
    }));

    commands.append(FilledRect::new(FilledRectParameter {
        scene: GAME.scene(),
        css_color: "#ffff00".to_string(),
        width: 300.,
        height: 30.,
        touchable: true,
    }));
}

fn move_system(
    mut rects: Query<&mut Transform, With<AkashicEntityId>>
) {
    for mut t in rects.iter_mut() {
        t.translation.y += 1.;
    }
}

fn read_point_down(
    mut er: EventReader<PointDown>,
    rects: Query<&AkashicEntityId>,
) {
    for event in er.iter() {
        let id = rects.iter().find(|r| **r == event.entity_id);
        console_log!("{:?}", id);
    }
}


fn read_scene_point_down_event(
    mut er: EventReader<ScenePointDown>
) {
    for e in er.iter() {
        console_log!("on point down scene! {:?}", e.point);
    }
}