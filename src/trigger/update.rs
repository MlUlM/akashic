use auto_delegate::delegate;
use crate::trigger::{Trigger, Void};

#[delegate]
pub trait UpdateHandler{
    /// Returns the Event trigger for frame elapsed.
    fn on_update(&self) -> Trigger<Void>;
}
