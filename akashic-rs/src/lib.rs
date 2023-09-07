mod game;
mod scene;
mod entity;

mod log;
mod trigger;
mod asset;
mod player;
pub mod shader;
pub mod param;

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
