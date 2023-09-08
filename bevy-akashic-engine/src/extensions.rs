use bevy::math::{Vec3};
use akashic_rs::prelude::CommonOffset;

pub trait AsVec3 {
    fn as_vec3(&self) -> Vec3;
}


impl AsVec3 for CommonOffset {
    #[inline(always)]
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x(), self.y(), 0.)
    }
}