use std::panic;

use bevy::prelude::*;
use bevy::time::TimePlugin;

use bevy_akashic::akashic::font::dynamic::DynamicFontBuilder;
use bevy_akashic::akashic::font::font_family::FontFamily;
use bevy_akashic::akashic::object2d::entity::AkashicEntityBuilder;
use bevy_akashic::akashic::object2d::entity::cacheable::label::LabelBuilder;
use bevy_akashic::akashic::prelude::FilledRectBuilder;
use bevy_akashic::event::message::AddMessageEvent;
use bevy_akashic::event::message::raise_event::RaiseEvent;
use bevy_akashic::event::message::request_raise_event::RaiseEventRequester;
use bevy_akashic::event::point_down::OnPointDown;
use bevy_akashic::prelude::*;
use bevy_akashic::prelude::object2d::touchable::Touchable;
use bevy_akashic::prelude::player_id::{PlayerId, SelfPlayerId};
use bevy_akashic::prelude::scene::GameScene;
use bevy_akashic::prelude::text::AkashicText;
use bevy_akashic::run_criteria::added_joined_as_streamer;

#[derive(Component)]
struct JoinButton;


#[derive(Component)]
struct StartButton;


#[derive(Resource, Default, Deref, DerefMut)]
struct JoinPlayers(Vec<PlayerId>);

#[derive(States, Hash, Eq, PartialEq, Debug, Copy, Clone, Default)]
enum GameState {
    #[default]
    Title,
    Game,
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_state::<GameState>()
        .init_resource::<JoinPlayers>()
        .add_plugins((
            TaskPoolPlugin::default(),
            FrameCountPlugin,
            TimePlugin
        ))
        .add_plugins(AkashicMinimumPlugins)
        .add_message_event::<PlayerId>()
        .add_systems(Startup, setup_join_button)
        .add_systems(Update, (
            click_join_button_system,
            setup_start_button.run_if(added_joined_as_streamer()),
            join_player_event_system,
            start_system
        ).run_if(in_state(GameState::Title)))
        .add_systems(OnExit(GameState::Title), all_destroy)
        .add_systems(OnEnter(GameState::Game), start_game_system)
        .run();
}


fn setup_join_button(mut commands: Commands) {
    let font = DynamicFontBuilder::new(FontFamily::new("sans-serif"), 32.).build();
    let label = LabelBuilder::new("参加する", font)
        .touchable(true)
        .local(true)
        .build();

    commands
        .spawn(label.into_bundle())
        .insert(JoinButton);
}


fn setup_start_button(mut commands: Commands) {
    let container = AkashicEntityBuilder::default()
        .touchable(true)
        .anchor_x(0.5)
        .anchor_y(0.5)
        .local(true)
        .width(300.)
        .height(100.)
        .x(100.)
        .y(-100.)
        .build();

    commands
        .spawn(container.into_bundle())
        .insert(StartButton)
        .with_children(|parent| {
            let background = FilledRectBuilder::new("blue", 300., 100.).build();
            parent.spawn(background.into_bundle());

            let font = DynamicFontBuilder::new(FontFamily::new("sans-serif"), 32.)
                .font_color("white")
                .build();
            let label = LabelBuilder::new("開始", font).build();

            parent.spawn(label.into_bundle());
        });
}


fn click_join_button_system(
    mut join_button: Query<(&mut Touchable, &mut AkashicText), (With<OnPointDown>, With<JoinButton>)>,
    self_id: Res<SelfPlayerId>,
    requester: RaiseEventRequester,
) {
    for (mut touchable, mut text) in join_button.iter_mut() {
        text.text = "参加済み".to_string();
        touchable.off();
        if let Some(player_id) = self_id.0.clone() {
            requester.raise_only_data(PlayerId(player_id));
        }
    }
}


fn start_system(
    mut state: ResMut<NextState<GameState>>,
    button: Query<Entity, (With<OnPointDown>, With<StartButton>)>,
) {
    if button.is_empty() { return; }
    state.set(GameState::Game);
}

fn join_player_event_system(
    mut joins: EventReader<RaiseEvent<PlayerId>>,
    mut players: ResMut<JoinPlayers>,
) {
    for p in joins.iter() {
        players.push(p.data.clone());
    }
}


fn all_destroy(
    mut commands: Commands,
    entities: Query<Entity, Without<GameScene>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}


fn start_game_system(
    mut commands: Commands,
    join_players: Res<JoinPlayers>,
) {
    let font = DynamicFontBuilder::new(FontFamily::new("sans-serif"), 50.).build();
    let label = LabelBuilder::new("ゲーム画面", font)
        .anchor_x(0.5)
        .anchor_y(0.5)
        .build();
    commands.spawn(label.into_bundle());

    let font = DynamicFontBuilder::new(FontFamily::new("sans-serif"), 32.).build();
    let label = LabelBuilder::new(format!("プレイヤー数: {}", join_players.len()), font)
        .anchor_x(0.5)
        .anchor_y(0.5)
        .y(-300.)
        .build();
    commands.spawn(label.into_bundle());
}