use auto_delegate::delegate;
use crate::trigger::{Trigger, Void};

#[delegate]
pub trait UpdateHandler{
    fn on_update(&self) -> Trigger<Void>;
}
