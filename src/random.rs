use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Clone, Debug)]
    pub type RandomGenerator;


    /// Generates a random number greater than or equal to 0 and less than 1.
    #[wasm_bindgen(method)]
    pub fn generate(this: &RandomGenerator) -> f32;
}