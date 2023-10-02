use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C"{
    #[derive(Clone, Debug)]
    pub type Player;

    #[wasm_bindgen(js_namespace = g, method, getter)]
    pub fn id(this: &Player) -> Option<String>;
}





