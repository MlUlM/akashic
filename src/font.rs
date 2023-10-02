use wasm_bindgen::prelude::wasm_bindgen;

pub mod dynamic;
pub mod font_family;
pub mod font_weight_string;
pub mod surface_atlas_set;
pub mod bitmap;


#[wasm_bindgen]
extern "C"{
    #[derive(Clone, Debug)]
    pub type Font;
}

