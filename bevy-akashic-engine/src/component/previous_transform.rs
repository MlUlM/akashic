use bevy::prelude::{Component, Transform};

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub(crate) struct PreviousTransform(pub(crate) Transform);

impl PartialEq<Transform> for PreviousTransform {
    #[inline(always)]
    fn eq(&self, other: &Transform) -> bool {
        &self.0 == other
    }
}


impl From<Transform> for PreviousTransform {
    fn from(value: Transform) -> Self {
        Self(value)
    }
}