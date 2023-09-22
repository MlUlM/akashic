use std::fmt::Debug;
use bevy::prelude::{Deref, Resource};
use akashic::game::GAME;
use akashic::random::RandomGenerator;

#[derive(Resource, Debug, Deref)]
pub struct AkashicRandomGenerator(RandomGenerator);


impl Default for AkashicRandomGenerator {
    fn default() -> Self {
        Self(GAME.random())
    }
}


/// ## Safety
///
/// シングルスレッドにおける実行環境のみを想定しているため将来的にマルチスレッドに対応された場合
/// 修正する必要があります。
unsafe impl Send for AkashicRandomGenerator {}

unsafe impl Sync for AkashicRandomGenerator {}


#[derive(Resource, Debug, Deref)]
pub struct AkashicLocalRandomGenerator(RandomGenerator);

impl Default for AkashicLocalRandomGenerator {
    fn default() -> Self {
        Self(GAME.local_random())
    }
}


/// ## Safety
///
/// シングルスレッドにおける実行環境のみを想定しているため将来的にマルチスレッドに対応された場合
/// 修正する必要があります。
unsafe impl Send for AkashicLocalRandomGenerator {}

unsafe impl Sync for AkashicLocalRandomGenerator {}
