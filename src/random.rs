use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Clone, Debug)]
    pub type RandomGenerator;


    /// 0以上1未満の範囲で乱数を生成します。
    #[wasm_bindgen(method)]
    pub fn generate(this: &RandomGenerator) -> f32;
}