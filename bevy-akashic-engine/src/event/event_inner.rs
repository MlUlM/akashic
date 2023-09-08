use bevy::prelude::{Entity, Event};
use akashic_rs::player::Player;
use akashic_rs::prelude::CommonOffset;
use akashic_rs::trigger::AkashicEventBase;


#[derive(Debug, Event)]
pub(crate) struct EventInner<B: AkashicEventBase>{
    base: B,
    button: Option<u8>,
    event_flags: Option<u8>,
    local: Option<bool>,
    target: Option<Option<Entity>>,
    player: Option<Option<Player>>,
    point: Option<CommonOffset>,
    pointer_id: Option<f32>
}
