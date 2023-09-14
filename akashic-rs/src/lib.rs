pub mod game;
pub mod scene;
pub mod entity;

pub mod log;
pub mod trigger;
pub mod asset;
pub mod player;
pub mod shader;
pub mod param;
pub mod font;
pub mod event;
pub mod random;

pub mod prelude {
    pub use crate::asset::*;
    pub use crate::console_log;
    pub use crate::entity::prelude::*;
    pub use crate::game::prelude::*;
    pub use crate::log::*;
    pub use crate::player::*;
    pub use crate::scene::prelude::*;
    pub use crate::trigger::prelude::*;
}
