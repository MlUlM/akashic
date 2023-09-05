mod game;
mod scene;
mod entity;

mod log;
mod object_2d_parameter;
mod trigger;
mod asset;
mod player;

pub mod prelude {
    pub use crate::console_log;
    pub use crate::asset::*;
    pub use crate::game::prelude::*;
    pub use crate::scene::prelude::*;
    pub use crate::log::*;
    pub use crate::player::*;
    pub use crate::trigger::prelude::*;
    pub use crate::entity::prelude::*;
    pub use crate::object_2d_parameter::*;
}
