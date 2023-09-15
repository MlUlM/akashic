use crate::asset::ImageAsset;
use crate::asset::surface::Surface;

#[derive(Debug, Clone)]
pub enum Src {
    Surface(Surface),
    ImageAsset(ImageAsset),
}




