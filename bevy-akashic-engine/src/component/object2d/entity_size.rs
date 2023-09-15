use bevy::math::Vec2;
use bevy::prelude::{Component, Deref, DerefMut};
use crate::prelude::object2d::entity::EntityProperties;

#[derive(Component, Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct AkashicEntitySize(Vec2);


impl AkashicEntitySize {
    #[inline(always)]
    pub(crate) fn new(size: &EntityProperties) -> Self {
        Self(Vec2::new(size.width, size.height))
    }


    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.0.x
    }


    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.0.y
    }
}