use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = g)]
extern {
    /// GameState is an object used in ranking mode.
    ///
    /// The [`score`](GameState::score) in this object is reflected in the ranking score.
    #[derive(Clone, Debug)]
    pub type GameState;

    /// Returns the current score.
    #[wasm_bindgen(method, getter, js_name = score)]
    pub fn score(this: &GameState) -> isize;


    /// Set the score.
    #[wasm_bindgen(method, setter, js_name = score)]
    pub fn set_score(this: &GameState, score: isize);
}


impl GameState {
    pub(crate) fn empty() -> Self {
        let o = Object::new();
        let attr = Object::new();
        js_sys::Reflect::set(&attr, &"writable".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&attr, &"value".into(), &0.into()).unwrap();

        js_sys::Reflect::define_property(&o, &"score".into(), &attr).unwrap();

        o.unchecked_into()
    }
}