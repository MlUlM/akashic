use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C"{
    #[derive(Clone, Debug)]
    pub type SurfaceAtlasSet;
}