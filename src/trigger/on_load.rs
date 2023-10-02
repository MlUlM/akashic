use crate::scene::Scene;
use crate::trigger::Trigger;

pub trait OnLoadHandler{
    fn on_load(&self) -> Trigger<Scene>;
}




