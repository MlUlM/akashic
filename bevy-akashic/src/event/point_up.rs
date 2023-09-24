use bevy::prelude::{Component, Deref};

use akashic::event::point::point_up::PointUpEvent;

use crate::event::event_inner::PointDeltaEventInner;

#[derive(Debug, Component, Clone, Deref)]
pub struct OnPointUp(PointDeltaEventInner<PointUpEvent>);


impl OnPointUp {
    #[inline(always)]
    pub(crate) fn new(
        native_event: PointUpEvent,
        half_game_width: f32,
        half_game_height: f32,
    ) -> Self {
        Self(PointDeltaEventInner::new(native_event, half_game_width, half_game_height))
    }
}

