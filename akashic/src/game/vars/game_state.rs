use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = g)]
extern {
    #[derive(Clone, Debug)]
    pub type GameState;

    #[wasm_bindgen(method, getter, js_name = score)]
    pub fn score(this: &GameState) -> isize;

    #[wasm_bindgen(method, setter, js_name = score)]
    pub fn set_score(this: &GameState, score: isize);
}


impl GameState {
    pub(crate) fn empty() -> Self {
        let o = Object::new();
        let attr = Object::new();
        js_sys::Reflect::set(&attr, &"writable".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&attr, &"value".into(), &0.into()).unwrap();

        js_sys::Reflect::define_property(&o, &"score".into(), &attr).expect("TODO: panic message");

        o.unchecked_into()
    }
}