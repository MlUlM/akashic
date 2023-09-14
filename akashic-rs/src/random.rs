use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Clone, Debug)]
    pub type RandomGenerator;


    #[wasm_bindgen(method)]
    /// 乱数を生成する。 0 以上 1 未満の数値を返す。
    pub fn generate(this: &RandomGenerator) -> f32;
}