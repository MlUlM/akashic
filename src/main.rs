use std::panic;

use bevy::app::{App, Startup, Update};
use bevy::core::{FrameCount, FrameCountPlugin};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Changed, Commands, Component, Event, EventReader, Query, Res, ResMut, Resource, Timer, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::time::{Time, TimePlugin, TimerMode};

use bevy_akashic::akashic::console_log;
use bevy_akashic::akashic::font::bitmap::BitmapFontBuilder;
use bevy_akashic::akashic::object2d::entity::cacheable::label::LabelBuilder;
use bevy_akashic::akashic::object2d::Object2D;
use bevy_akashic::akashic::prelude::{FilledRectBuilder, SpriteBuilder};
use bevy_akashic::component::object2d::entity_size::AkashicEntitySize;
use bevy_akashic::event::message::AddMessageEvent;
use bevy_akashic::event::message::raise_event::RaiseEvent;
use bevy_akashic::event::message::request_raise_event::RaiseEventRequester;
use bevy_akashic::event::point_down::OnPointDown;
use bevy_akashic::event::point_move::OnPointMove;
use bevy_akashic::event::point_up::OnPointUp;
use bevy_akashic::plugin::asset::AkashicAssetServer;
use bevy_akashic::prelude::*;
use bevy_akashic::prelude::object2d::entity::filled_rect::CssColor;
use bevy_akashic::prelude::object2d::touchable::Touchable;
use bevy_akashic::prelude::scene::GameScene;
use bevy_akashic::prelude::text::AkashicText;
use bevy_akashic::resource::game::GameInfo;


#[derive(Component, Debug)]
struct Angel;

#[derive(Component, Debug)]
struct Shot;


#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct TestMessageEvent(String);


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(MyTimer(Timer::from_seconds(3., TimerMode::Repeating)))
        .add_message_event::<TestMessageEvent>()
        .add_plugins((
            FrameCountPlugin,
            TimePlugin
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_hovering_system,
            label_sytem,
            spawn_player_system,
            point_down,
            point_move,
            point_up,
            read_raise_events
        ))
        .run();
}

#[derive(Resource)]
struct MyTimer(Timer);


fn spawn_player_system(
    mut timer: ResMut<MyTimer>,
    mut touches: Query<&mut Touchable, With<Angel>>,
    mut colors: Query<&mut CssColor>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut c in colors.iter_mut() {
            c.set_rgba(1., 0., 0., 1.);
        }
        for mut t in touches.iter_mut() {
            t.toggle();
        }
    }
}


fn label_sytem(
    mut labels: Query<&mut AkashicText>,
    touch: Query<&Touchable, (With<Angel>, Changed<Touchable>)>,
) {
    for t in touch.iter() {
        for mut l in labels.iter_mut() {
            l.text = if t.get() {
                "おん".to_string()
            } else {
                "おふ".to_string()
            };
        }
    }
}


fn setup(mut commands: Commands, server: Res<AkashicAssetServer>, game_size: Res<GameInfo>) {
    console_log!("SETUP");

    let akashic_entity = bevy_akashic::akashic::object2d::entity::AkashicEntity::default();
    let font_glyphs = server.text_by_id("font_glyphs");
    let src = server.image_by_id("font");
    let label = LabelBuilder::new(
        "あかさたな",
        BitmapFontBuilder::new(src)
            .glyph_info(&font_glyphs.data())
            .build(),
    )
        .build();

    let player_image_asset = server.image_by_id("player");
    let player = SpriteBuilder::new(player_image_asset)
        .local(true)
        .touchable(true)
        .build();

    player.set_x((game_size.width() - player.width()) / 2.);
    player.set_y((game_size.height() - player.height()) / 2.);
    player.set_angle(45.);

    commands
        .spawn(akashic_entity.into_bundle())
        .with_children(|parent| {
            parent.spawn(label.into_bundle());
            parent.spawn(player.into_bundle());
            parent.spawn(FilledRectBuilder::new("blue", 32., 32.).build().into_bundle());
        })
        .insert(Angel);
}


fn player_hovering_system(
    mut player: Query<(&mut Transform, &AkashicEntitySize), With<Angel>>,
    game_info: Res<GameInfo>,
    frames: Res<FrameCount>,
) {
    for (mut transform, size) in player.iter_mut() {
        transform.translation.y = (game_info.height() - size.height()) / 2. + ((frames.0 as f32) % (game_info.fps() * 10.) / 4.).sin() * 10.;
    }
}


fn point_down(
    raise_event_requester: RaiseEventRequester,
    player: Query<&OnPointDown, With<Angel>>,
) {
    for p in player.iter() {
        raise_event_requester.raise_only_data(TestMessageEvent("HELLO".to_string()));
        console_log!("down = {:?}", p.point());
    }
}


fn point_move(
    tests: Query<&OnPointMove, With<GameScene>>
) {
    for p in tests.iter() {
        console_log!("move = {:?}", p.prev_delta());
    }
}


fn point_up(
    tests: Query<&OnPointUp, With<GameScene>>
) {
    for p in tests.iter() {
        console_log!("up = {:?}", p.start_delta());
    }
}


fn read_raise_events(
    mut er: EventReader<RaiseEvent<TestMessageEvent>>
) {
    for event in er.iter() {
        console_log!("{:?}", event);
    }
}