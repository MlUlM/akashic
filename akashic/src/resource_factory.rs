use wasm_bindgen::prelude::wasm_bindgen;
use crate::asset::binary::BinaryAsset;
use crate::asset::surface::Surface;

#[wasm_bindgen(js_namespace=g)]
extern {
    #[derive(Debug, Clone)]
    pub type ResourceFactory;
    
    #[wasm_bindgen(method, js_name=createSurface)]
    pub fn create_surface(this: &ResourceFactory, width: f32, height: f32) -> Surface;


    #[wasm_bindgen(method, js_name=createBinaryAsset)]
    pub fn create_binary_asset(this: &ResourceFactory, id: String, path: String) -> BinaryAsset;
}