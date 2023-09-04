use crate::asset::ImageAsset;
use crate::asset::surface::Surface;
pub trait IntoSrc{
    fn into_src(self) -> Src;
}



#[derive(Debug)]
pub enum Src {
    Surface(Surface),
    ImageAsset(ImageAsset),
}




