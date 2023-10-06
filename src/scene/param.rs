use derive_builder::Builder;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::game::Game;
use crate::scene::Scene;

#[non_exhaustive]
#[wasm_bindgen(getter_with_clone)]
#[derive(Builder)]
#[builder(
name = "SceneBuilder",
build_fn(private, name = "fallible_build")
)]
pub struct SceneParam {
    #[builder(default)]
    pub game: Game,

    #[wasm_bindgen(js_name = assetIds)]
    #[builder(setter(custom), default)]
    pub asset_ids: Option<Box<[JsString]>>,

    #[wasm_bindgen(js_name = assetPaths)]
    #[builder(setter(custom), default)]
    pub asset_paths: Option<Box<[JsString]>>,

    #[wasm_bindgen(js_name = storageKeys)]
    #[builder(setter(into, strip_option), default)]
    pub storage_keys: Option<Box<[JsValue]>>,

    #[builder(default)]
    pub local: LocalTickMode,

    #[builder(setter(into, strip_option), default)]
    pub name: Option<JsString>,

    #[builder(default)]
    pub seethrough: bool,

    #[wasm_bindgen(js_name = tickGenerationMode)]
    #[builder(default)]
    pub tick_generation_mode: TickGenerationMode,
}


impl SceneBuilder {
    #[inline]
    pub fn assets_ids(&mut self, assets_ids: Vec<&str>) -> &mut Self {
        self.asset_ids = Some(Some(assets_ids.into_iter().map(JsString::from).collect()));
        self
    }

    #[inline]
    pub fn assets_paths(&mut self, asset_paths: Vec<&str>) -> &mut Self {
        self.asset_paths = Some(Some(asset_paths.into_iter().map(JsString::from).collect()));
        self
    }

    #[inline]
    pub fn build(&self) -> Scene {
        Scene::new(self.fallible_build().unwrap())
    }
}



///
#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum LocalTickMode {
    NonLocal = "non-local",
    FullLocal = "full-local",
    InterpolateLocal = "interpolate-local",
}


#[allow(clippy::derivable_impls)]
impl Default for LocalTickMode {
    fn default() -> Self {
        LocalTickMode::NonLocal
    }
}


#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TickGenerationMode {
    ByClock = "by-clock",

    Manual = "manual",
}


#[allow(clippy::derivable_impls)]
impl Default for TickGenerationMode {
    fn default() -> Self {
        Self::ByClock
    }
}