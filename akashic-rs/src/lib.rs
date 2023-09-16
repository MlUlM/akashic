pub mod game;
pub mod scene;

pub mod log;
pub mod trigger;
pub mod asset;
pub mod player;
pub mod shader;
pub mod parent;
pub mod font;
pub mod event;
pub mod random;
pub mod error;
pub mod object2d;
pub mod option_number;
pub mod util;
pub mod common_offset;

pub mod prelude {
    pub use crate::asset::*;
    pub use crate::console_log;
    pub use crate::game::prelude::*;
    pub use crate::log::*;
    pub use crate::object2d::entity::prelude::*;
    pub use crate::player::*;
    pub use crate::scene::prelude::*;
    pub use crate::trigger::prelude::*;
    pub use crate::{
        common_offset::CommonOffset,
    };
}
