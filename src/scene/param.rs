use js_sys::JsString;
use wasm_bindgen::{JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::game::Game;


#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Clone)]
#[non_exhaustive]
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
    pub fn builder(game: Game) -> SceneParameterObjectBuilder {
        SceneParameterObjectBuilder::new(game)
    }
}


// TODO: SCENEのパラメータを定義しなおす
#[allow(unused)]
#[derive(Default)]
pub struct SceneParameterObjectBuilder {
    game: Game,
    asset_ids: Option<Vec<&'static str>>,
    asset_paths: Option<Vec<String>>,
    storage_keys: Option<Vec<String>>,
    local: bool,
    name: Option<String>,
}


impl SceneParameterObjectBuilder {
    #[inline]
    pub fn new(game: Game) -> Self {
        Self {
            game,
            ..Default::default()
        }
    }


    #[inline]
    pub fn asset_ids(mut self, asset_ids: Vec<&'static str>) -> SceneParameterObjectBuilder {
        self.asset_ids = Some(asset_ids);
        self
    }


    #[inline]
    pub fn build(self) -> SceneParameterObject {
        SceneParameterObject {
            game: self.game,
            asset_ids: self.asset_ids.map(|ids| ids.into_iter().map(JsString::from).collect()),
            asset_paths: self.asset_paths.map(|p| p.into_iter().map(JsString::from).collect()),
            name: self.name.map(JsString::from),
            ..Default::default()
        }
    }
}

