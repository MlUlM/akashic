use bevy::prelude::{Component, Deref};

use crate::plugin::event::AkashicPointUpEvent;

#[derive(Debug, Component, Clone, Deref)]
pub struct OnPointUp(AkashicPointUpEvent);


impl From<AkashicPointUpEvent> for OnPointUp {
    #[inline]
    fn from(value: AkashicPointUpEvent) -> Self {
        Self(value)
    }
}
