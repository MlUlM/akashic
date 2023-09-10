use crate::event::join::JoinEvent;
use crate::prelude::Trigger;

pub trait JoinHandler {
    fn on_join(&self) -> Trigger<JoinEvent>;
}
