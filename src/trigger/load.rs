use crate::scene::Scene;
use crate::trigger::Trigger;

pub trait LoadHandler {
    /// Returns the trigger dealing with Loading.
    ///
    /// This event trigger only occurs once when the scene possessing this trigger finished loading assets.
    fn on_load(&self) -> Trigger<Scene>;
}




