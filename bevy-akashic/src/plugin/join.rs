use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, EventWriter, PreUpdate, Res};

use akashic::game::GAME;
use akashic::trigger::join::JoinHandler;

use crate::event::AkashicEventQueue;
use crate::event::join::JoinEvent;
use crate::prelude::player_id::PlayerId;
use crate::resource::join::{JoinedAsListener, JoinedAsStreamer};

pub struct AkashicJoinEventPlugin;

impl Plugin for AkashicJoinEventPlugin {
    fn build(&self, app: &mut App) {
        let join_event_queue = AkashicEventQueue::<JoinEvent>::default();

        app
            .add_event::<JoinEvent>()
            .insert_resource(join_event_queue.clone())
            .add_systems(PreUpdate, read_join_event_queue_system);

        GAME
            .on_join()
            .add(move |event| {
                join_event_queue.push(JoinEvent::new(event));
            });
    }
}

fn read_join_event_queue_system(
    mut commands: Commands,
    mut ew: EventWriter<JoinEvent>,
    queue: Res<AkashicEventQueue<JoinEvent>>,
) {
    while let Some(event) = queue.pop_front() {
        insert_joined_resource_target_with_nicolive(&mut commands, &event);
        ew.send(event);
    }
}

fn insert_joined_resource_target_with_nicolive(commands: &mut Commands, join_event: &JoinEvent) {
    let Some(streamer_id) = join_event.player().id() else { return; };
    let Some(self_id) = GAME.self_id() else { return; };

    // JoinEventが発火されるのはニコ生の場合配信者だけらしいため、
    // 自身のIDと同じ場合は配信者となる
    if self_id == streamer_id {
        commands.insert_resource(JoinedAsStreamer(PlayerId(self_id)));
    } else {
        commands.insert_resource(JoinedAsListener(PlayerId(self_id)));
    }
}
