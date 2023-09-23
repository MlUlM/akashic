use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace=g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type CommonOffset;

    #[wasm_bindgen(getter, method)]
    pub fn x(this: &CommonOffset) -> f32;

    #[wasm_bindgen(getter, method)]
    pub fn y(this: &CommonOffset) -> f32;
}