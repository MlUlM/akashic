use wasm_bindgen::prelude::wasm_bindgen;
use crate::game::vars::game_state::GameState;

pub mod game_state;

#[wasm_bindgen]
extern {
    #[derive(Debug, Clone)]
    pub type Vars;

    #[wasm_bindgen(method, getter, js_name = gameState)]
    fn _game_state(this: &Vars) -> Option<GameState>;

    #[wasm_bindgen(method, setter, js_name = gameState)]
    fn _set_game_state(this: &Vars, game_state: GameState);
}


impl Vars {
    pub fn game_state(&self) -> GameState {
        if let Some(game_state) = self._game_state() {
            game_state
        } else {
            let game_state = GameState::empty();
            self._set_game_state(game_state.clone());
            game_state
        }
    }
}

