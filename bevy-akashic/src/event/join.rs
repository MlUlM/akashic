use bevy::prelude::Event;
use akashic::player::Player;
use crate::SharedObject;

#[derive(Event, Debug)]
pub struct JoinEvent(SharedObject<akashic::event::join::JoinEvent>);


impl JoinEvent {
    #[inline]
    pub fn new(native_join_event: akashic::event::join::JoinEvent) -> Self {
        Self(SharedObject::new(native_join_event))
    }


    #[inline]
    pub fn player(&self) -> Player {
        self.0.lock().player()
    }
}


