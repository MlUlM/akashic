use bevy::prelude::{Component, Deref, DerefMut, Resource};

#[derive(Debug,  Eq, PartialEq, Hash, Deref, DerefMut, Clone, Component, Resource)]
pub struct PlayerId(pub(crate) String);



