
use crate::trigger::{Trigger, Void};


pub trait UpdateHandler{
    fn on_update(&self) -> Trigger<Void>;
}
