use bevy::prelude::Resource;

use akashic_rs::game::GAME;
use akashic_rs::game::vars::game_state::GameState;

#[derive(Debug, Resource)]
pub struct AkashicGameState {
    inner: GameState,
    cache: isize,
}


impl AkashicGameState {
    #[inline(always)]
    pub fn score(&self) -> isize {
        self.cache
    }

    #[inline(always)]
    pub fn set_score(&mut self, score: isize) {
        self.inner.set_score(score);
        self.cache = score;
    }

    #[inline(always)]
    pub fn add_score_by(&mut self, add_score: isize) {
        self.set_score(self.cache + add_score);
        self.cache += add_score;
    }

    #[inline(always)]
    pub fn increment_score(&mut self) {
        self.add_score_by(1);
    }
}


unsafe impl Sync for AkashicGameState {}

unsafe impl Send for AkashicGameState {}


impl Default for AkashicGameState {
    fn default() -> Self {
        Self {
            inner: GAME.vars().game_state(),
            cache: 0,
        }
    }
}