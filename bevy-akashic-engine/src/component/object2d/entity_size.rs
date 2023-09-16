use bevy::math::Vec2;
use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Component, Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct AkashicEntitySize(Vec2);


impl AkashicEntitySize {
    #[inline(always)]
    pub(crate) fn new(size: Vec2) -> Self {
        Self(size)
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