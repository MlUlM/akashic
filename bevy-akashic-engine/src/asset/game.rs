use bevy::math::Vec2;
use bevy::prelude::Resource;
use akashic_rs::prelude::GAME;


#[derive(Resource, Default)]
pub struct GameInfo;


impl GameInfo {
    #[inline(always)]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }


    #[inline(always)]
    pub fn width(&self) -> f32 {
        GAME.width()
    }


    #[inline(always)]
    pub fn height(&self) -> f32 {
        GAME.height()
    }


    #[inline(always)]
    pub fn age(&self) -> f32 {
        GAME.age()
    }

    #[inline(always)]
    pub fn fps(&self) -> f32 {
        GAME.fps()
    }
}