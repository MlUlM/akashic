use js_sys::JsString;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::game::Game;

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct SceneParameterObject {
    pub game: Game,

    #[wasm_bindgen(js_name = assetIds)]
    pub asset_ids: Option<Box<[JsString]>>,

    #[wasm_bindgen(js_name = assetPaths)]
    pub asset_paths: Option<Box<[JsString]>>,

    #[wasm_bindgen(js_name = storageKeys)]
    pub storage_keys: Option<Box<[JsValue]>>,

    pub local: bool,

    pub name: Option<JsString>,
}


impl SceneParameterObject {
    pub fn builder(game: Game) -> Builder{
        Builder::new(game)
    }
}

#[derive(Default)]
pub struct Builder{
    pub game: Game,
    pub asset_ids: Option<Vec<String>>,
    pub asset_paths: Option<Vec<String>>,
    pub storage_keys: Option<Vec<String>>,
    pub local: bool,
    pub name: Option<String>,
}


impl Builder {
    pub fn new(game: Game) -> Self{
        Self{
            game,
            ..Default::default()
        }
    }


    pub fn build(self) -> SceneParameterObject{
        SceneParameterObject{
            game: self.game,
            asset_ids: self.asset_ids.map(|ids|ids.into_iter().map(JsString::from).collect()),
            ..Default::default()
        }
    }
}

