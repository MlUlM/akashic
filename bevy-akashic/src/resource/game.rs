use bevy::math::Vec2;
use bevy::prelude::Resource;
use once_cell::sync::OnceCell;

use akashic::prelude::GAME;


#[derive(Resource, Default)]
pub struct GameInfo {
    size: OnceCell<Vec2>,
    width: OnceCell<f32>,
    height: OnceCell<f32>,
    fps: OnceCell<f32>,
    self_id: OnceCell<String>,
    half_width: OnceCell<f32>,
    half_height: OnceCell<f32>,
}


impl GameInfo {
    #[inline(always)]
    pub fn size(&self) -> Vec2 {
        *self.size.get_or_init(|| Vec2::new(self.width(), self.height()))
    }


    #[inline(always)]
    pub fn width(&self) -> f32 {
        *self.width.get_or_init(|| GAME.width())
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        *self.height.get_or_init(|| GAME.height())
    }


    #[inline(always)]
    pub fn fps(&self) -> f32 {
        *self.fps.get_or_init(|| GAME.fps())
    }


    #[inline(always)]
    pub fn self_id(&self) -> String {
        self.self_id.get_or_init(|| GAME.self_id().unwrap()).clone()
    }

    #[inline]
    pub fn age(&self) -> f32 {
        GAME.age()
    }


    #[inline]
    pub fn half_width(&self) -> f32 {
        *self.half_width.get_or_init(|| self.width() / 2.)
    }


    #[inline]
    pub fn half_height(&self) -> f32 {
        *self.half_height.get_or_init(|| self.height() / 2.)
    }
}


