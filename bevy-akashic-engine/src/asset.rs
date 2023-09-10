use std::collections::HashMap;

use bevy::prelude::Resource;

use akashic_rs::prelude::{AudioAsset, GAME};
use akashic_rs::prelude::ImageAsset;

use crate::SharedObject;

#[derive(Resource, Debug)]
pub struct AkashicAssetServer {
    images: HashMap<String, SharedObject<ImageAsset>>,
    audios: HashMap<String, SharedObject<AudioAsset>>,
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
}


impl Default for AkashicAssetServer {
    fn default() -> Self {
        let assets = GAME.scene().asset();

        AkashicAssetServer {
            images: asset_map(assets.get_all_images_map("/image/*.png")),
            audios: asset_map(assets.get_all_audios_map("/audio/*")),
        }
    }
}


#[inline]
fn asset_map<T>(map: HashMap<String, T>) -> HashMap<String, SharedObject<T>> {
    map
        .into_iter()
        .map(|(path, asset)| (path, SharedObject::new(asset)))
        .collect()
}
