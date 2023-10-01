use bevy::prelude::{Component, Deref, Event, Reflect};

use crate::plugin::event::AkashicPointDownEvent;

#[derive(Debug, Clone, Event, Deref, Component)]
pub struct OnPointDown(AkashicPointDownEvent);

impl From<AkashicPointDownEvent> for OnPointDown {
    #[inline]
    fn from(value: AkashicPointDownEvent) -> Self {
        Self(value)
    }
}
