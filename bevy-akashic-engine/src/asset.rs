pub mod game;

use bevy::prelude::Resource;
use akashic_rs::prelude::{AssetAccessor, AudioAsset, GAME};
use akashic_rs::prelude::ImageAsset;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub struct AkashicAssetServer;


impl AkashicAssetServer {
    #[inline]
    pub fn get_image_by_id(
        &self,
        asset_id: impl Into<String>,
    ) -> ImageAsset {
        self.accessor().get_image_by_id(asset_id.into())
    }


    #[inline]
    pub fn get_audio_by_id(
        &self,
        asset_id: impl Into<String>,
    ) -> AudioAsset {
        self.accessor().get_audio_by_id(asset_id.into())
    }


    fn accessor(&self) -> AssetAccessor {
        GAME
            .scene()
            .asset()
    }
}




