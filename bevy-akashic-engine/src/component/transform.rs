use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Debug)]
pub struct AkashicTransform{
    pub transition: Vec3
}

