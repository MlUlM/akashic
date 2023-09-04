use akashic_rs::prelude::AudioAsset;
use crate::SharedObject;

mod play;

pub mod prelude{
    pub use crate::command::audio::play::*;
}


pub(crate) type SharedAudioAsset = SharedObject<AudioAsset>;
