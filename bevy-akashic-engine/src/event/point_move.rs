use bevy::prelude::{Component, Deref};

use akashic_rs::event::point::point_move::PointMoveEvent;

use crate::event::event_inner::PointDeltaEventInner;

#[derive(Debug, Component, Deref)]
pub struct OnPointMove(PointDeltaEventInner<PointMoveEvent>);


impl OnPointMove {
    #[inline(always)]
    pub(crate) fn new(native_event: PointMoveEvent) -> Self {
        Self(PointDeltaEventInner::new(native_event))
    }
}




