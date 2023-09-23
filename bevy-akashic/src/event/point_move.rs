use bevy::prelude::{Component, Deref};

use akashic::event::point::point_move::PointMoveEvent;

use crate::event::event_inner::PointDeltaEventInner;

#[derive(Debug, Component, Deref)]
pub struct OnPointMove(PointDeltaEventInner<PointMoveEvent>);


impl OnPointMove {
    #[inline(always)]
    pub(crate) fn new(
        native_event: PointMoveEvent,
        half_game_width: f32,
        half_game_height: f32
    ) -> Self {
        Self(PointDeltaEventInner::new(native_event, half_game_width, half_game_height))
    }
}




