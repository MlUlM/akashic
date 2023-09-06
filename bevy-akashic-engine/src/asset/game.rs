use bevy::math::Vec2;
use bevy::prelude::Resource;
use akashic_rs::prelude::GAME;


#[derive(Resource)]
pub struct GameInfo{
    pub size: Vec2,
    pub width: f32,
    pub height: f32,
    pub age: f32,
    pub fps: f32
}


impl Default for  GameInfo {
    #[inline]
    fn default() -> Self {
        let width = GAME.width();
        let height = GAME.height();

        GameInfo{
            size: Vec2::new(width, height),
            width,
            height,
            age: GAME.age(),
            fps: GAME.fps()
        }
    }
}