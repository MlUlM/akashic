use std::path::{Path, PathBuf};

use bevy::app::{App, Plugin};
use bevy::asset::{AssetIo, AssetIoError, AssetServer, BoxedFuture, ChangeWatcher, FileType, Metadata};
use wasm_bindgen::prelude::wasm_bindgen;

pub struct AkashicAssetIoPlugin;


impl Plugin for AkashicAssetIoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetServer::new(AkashicAssetIo));
    }
}

struct AkashicAssetIo;


impl AssetIo for AkashicAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, anyhow::Result<Vec<u8>, AssetIoError>> {
        Box::pin(async move {
            let Some(file_path) = path.to_str().map(|p| p.to_string()) else {
                return Err(AssetIoError::NotFound(path.to_path_buf()));
            };

            let Some(binary_data) = read_asset_binaries(file_path) else {
                return Err(AssetIoError::NotFound(path.to_path_buf()));
            };

            Ok(binary_data.into_vec())
        })
    }

    fn read_directory(&self, path: &Path) -> anyhow::Result<Box<dyn Iterator<Item=PathBuf>>, AssetIoError> {
        Ok(Box::new(vec![
            path.to_path_buf()
        ].into_iter()))
    }

    fn get_metadata(&self, path: &Path) -> anyhow::Result<Metadata, AssetIoError> {
        if path.is_dir() {
            Ok(Metadata::new(FileType::Directory))
        } else {
            Ok(Metadata::new(FileType::File))
        }
    }

    fn watch_path_for_changes(&self, _: &Path, _: Option<PathBuf>) -> anyhow::Result<(), AssetIoError> {
        Ok(())
    }


    fn watch_for_changes(&self, _: &ChangeWatcher) -> anyhow::Result<(), AssetIoError> {
        Ok(())
    }
}


unsafe impl Send for AkashicAssetIo {}

unsafe impl Sync for AkashicAssetIo {}


#[wasm_bindgen(js_namespace = g)]
extern {
    fn read_asset_binaries(path: String) -> Option<Box<[u8]>>;
}