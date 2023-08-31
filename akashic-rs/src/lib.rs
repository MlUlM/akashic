pub mod game;
pub mod scene;

pub mod filled_rect;

pub mod entity;

mod log;
mod object_2d_parameter;
mod trigger;

pub mod prelude {
    pub use crate::console_log;
    pub use crate::filled_rect::{FilledRect, FilledRectParameter};
    pub use crate::game::{Game, GAME};
    pub use crate::scene::Scene;
    pub use crate::log::log;
    pub use crate::trigger::prelude::*;
    pub use crate::entity::{E, Entity};
}
