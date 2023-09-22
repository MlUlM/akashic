use bevy::prelude::{Component, Deref};
use akashic::event::point::point_up::PointUpEvent;

use crate::event::event_inner::PointDeltaEventInner;

#[derive(Debug, Component, Deref)]
pub struct OnPointUp(PointDeltaEventInner<PointUpEvent>);


impl OnPointUp {
    #[inline(always)]
    pub(crate) fn new(native_event: PointUpEvent) -> Self {
        Self(PointDeltaEventInner::new(native_event))
    }
}

