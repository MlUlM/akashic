use wasm_bindgen::prelude::wasm_bindgen;
use crate::game::vars::game_state::GameState;

pub mod game_state;

#[wasm_bindgen]
extern {
    /// `This value is not used by akashic-engine, so you can use it for any purpose.`
    ///
    ///
    /// The above is from official reference, so you might this that this value use is completely optional
    /// and that akashic engine will never set a value for this variable, but there are some exceptions.
    ///
    /// One of them is [`GameState`](GameState), which should be used to set score whe running in ranking mode.
    #[derive(Debug, Clone)]
    pub type Vars;

    #[wasm_bindgen(method, getter, js_name = gameState)]
    fn _game_state(this: &Vars) -> Option<GameState>;

    #[wasm_bindgen(method, setter, js_name = gameState)]
    fn _set_game_state(this: &Vars, game_state: GameState);
}


impl Vars {
    /// Returns the [`GameState`](GameState).
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

