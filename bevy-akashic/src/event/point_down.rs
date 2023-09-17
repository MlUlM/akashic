use bevy::prelude::{Component, Deref, Event};
use akashic_rs::prelude::PointDownEvent;

use crate::prelude::event_inner::PointEventInner;


#[derive(Debug, Event, Deref, Component)]
pub struct OnPointDown(PointEventInner<PointDownEvent>);

impl OnPointDown{
    #[inline]
    pub fn new(native_event: PointDownEvent) -> Self{
        Self(PointEventInner::new(native_event))
    }
}



