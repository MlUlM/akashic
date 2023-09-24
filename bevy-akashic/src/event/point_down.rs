use bevy::prelude::{Component, Deref, Event};

use akashic::prelude::PointDownEvent;

use crate::prelude::event_inner::PointEventInner;

#[derive(Debug, Clone, Event, Deref, Component)]
pub struct OnPointDown(PointEventInner<PointDownEvent>);

impl OnPointDown {
    #[inline]
    pub fn new(
        native_event: PointDownEvent,
        half_game_width: f32,
        half_game_height: f32,
    ) -> Self {
        Self(PointEventInner::new(native_event, half_game_width, half_game_height))
    }
}


