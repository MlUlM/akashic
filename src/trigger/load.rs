use crate::scene::Scene;
use crate::trigger::Trigger;

pub trait LoadHandler {
    fn on_load(&self) -> Trigger<Scene>;
}




