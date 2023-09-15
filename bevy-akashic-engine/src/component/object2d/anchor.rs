use bevy::prelude::Component;

use crate::prelude::object2d::entity::EntityProperties;

#[non_exhaustive]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Anchor {
    pub x: Option<f32>,
    pub y: Option<f32>,
}


impl Anchor {
    #[inline]
    pub(crate) fn new(properties: &EntityProperties) -> Self {
        Self {
            x: properties.anchor_x,
            y: properties.anchor_y,
        }
    }
}