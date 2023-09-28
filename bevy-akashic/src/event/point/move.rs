use bevy::prelude::{Component, Deref};

use crate::plugin::event::AkashicPointMoveEvent;

#[derive(Debug, Component, Clone, Deref)]
pub struct OnPointMove(AkashicPointMoveEvent);


impl From<AkashicPointMoveEvent> for OnPointMove {
    #[inline]
    fn from(value: AkashicPointMoveEvent) -> Self {
        Self(value)
    }
}

