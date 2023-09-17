use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

use akashic_rs::asset::audio::AudioAsset;
use akashic_rs::asset::image::ImageAsset;
use akashic_rs::asset::text::TextAsset;
use akashic_rs::prelude::GAME;

use crate::SharedObject;

pub struct AkashicAssetPlugin;


impl Plugin for AkashicAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AkashicAssetServer>();
    }
}


#[derive(Resource, Debug)]
pub struct AkashicAssetServer {
    images: HashMap<String, SharedObject<ImageAsset>>,
    audios: HashMap<String, SharedObject<AudioAsset>>,
    texts: HashMap<String, SharedObject<TextAsset>>,
}


impl AkashicAssetServer {
    #[inline]
    pub fn image_by_id(
        &self,
        asset_id: &str,
    ) -> ImageAsset {
        self.get_image_by_id(asset_id).unwrap()
    }

    #[inline]
    pub fn get_image_by_id(
        &self,
        asset_id: &str,
    ) -> Option<ImageAsset> {
        self
            .images
            .get(asset_id)
            .map(|o| o.lock().clone())
    }

    #[inline]
    pub fn audio_by_id(
        &self,
        asset_id: &str,
    ) -> AudioAsset {
        self.get_audio_by_id(asset_id).unwrap()
    }


    #[inline]
    pub fn get_audio_by_id(
        &self,
        asset_id: &str,
    ) -> Option<AudioAsset> {
        self
            .audios
            .get(asset_id)
            .map(|o| o.lock().clone())
    }


    #[inline]
    pub fn text_by_id(&self, asset_id: &str) -> TextAsset{
        self.get_text_by_id(asset_id).unwrap_or_else(|| panic!("Not found text asset; id={asset_id}"))
    }


    pub fn get_text_by_id(&self, asset_id: &str) -> Option<TextAsset>{
        self
            .texts
            .get(asset_id)
            .map(|o|o.lock().clone())
    }
}


impl Default for AkashicAssetServer {
    fn default() -> Self {
        let assets = GAME.scene().asset();

        AkashicAssetServer {
            images: convert_to_hash_map(assets.get_all_images_with_path_pattern("/image/*".to_string())),
            audios: convert_to_hash_map(assets.get_all_audios_with_path_pattern("/audio/*".to_string())),
            texts: convert_to_hash_map(assets.get_all_texts_with_path_pattern("/text/*".to_string())),
        }
    }
}

#[allow(clippy::boxed_local)]
fn convert_to_hash_map<A: akashic_rs::asset::Asset + Clone>(assets: Box<[A]>) -> HashMap<String, SharedObject<A>> {
    assets
        .iter()
        .map(|asset| (asset.id(), asset.clone()))
        .map(|(path, asset)| (path, SharedObject::new(asset.clone())))
        .collect()
}


