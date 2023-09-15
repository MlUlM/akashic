pub mod audio;

pub mod prelude {
    pub use crate::command::{IntoBundle, audio::prelude::*};
}

pub trait IntoBundle<B> {
    fn into_bundle(self) -> B;
}
