use bevy::math::Vec2;
use akashic_rs::prelude::CommonOffset;

pub trait AsVec2{
    fn as_vec2(&self) -> Vec2;
}


impl AsVec2 for CommonOffset {
    #[inline(always)]
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }
}