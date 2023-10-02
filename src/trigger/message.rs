use crate::event::message::MessageEvent;
use crate::prelude::Trigger;

pub trait MessageHandler{
    fn on_message(&self) -> Trigger<MessageEvent>;
}


