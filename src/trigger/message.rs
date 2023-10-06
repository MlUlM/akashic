use crate::event::message::MessageEvent;
use crate::prelude::Trigger;

pub trait MessageHandler{
    /// Returns the trigger dealing with [`MessageEvent`].
    fn on_message(&self) -> Trigger<MessageEvent>;
}


