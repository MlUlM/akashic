mod command;

mod plugin;
mod trigger;
mod component;

pub mod prelude {
    pub use crate::command::append::{AkashicCommandEx, Append};
    pub use crate::plugin::{AkashicPlugin, SceneLoadState};
    pub use akashic_rs::prelude::*;
    pub use crate::trigger::point_down::PointDown;
    pub use crate::component::prelude::*;
}
