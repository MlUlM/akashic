use bevy::prelude::Component;


#[derive(Component, Debug)]
pub struct GameScene(pub(crate) akashic::scene::Scene);


unsafe impl Send for GameScene {}

unsafe impl Sync for GameScene {}


