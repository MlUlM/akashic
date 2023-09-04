use std::ops::{Deref, DerefMut};

use bevy::math::Vec2;
use bevy::prelude::Component;

use akashic_rs::prelude::EntitySize;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct AkashicEntitySize(Vec2);


impl AkashicEntitySize {
    #[inline(always)]
    pub fn new(size: &impl EntitySize) -> Self {
        Self(Vec2::new(size.width(), size.height()))
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


impl Deref for AkashicEntitySize {
    type Target = Vec2;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for AkashicEntitySize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}