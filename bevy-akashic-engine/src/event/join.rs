use bevy::prelude::Event;
use akashic_rs::player::Player;
use akashic_rs::trigger::PointEventBase;
use crate::SharedObject;

#[derive(Event, Debug)]
pub struct JoinEvent(SharedObject<akashic_rs::event::join::JoinEvent>);


impl JoinEvent {
    #[inline]
    pub fn new(native_join_event: akashic_rs::event::join::JoinEvent) -> Self{
        Self(SharedObject::new(native_join_event))
    }


    #[inline]
    pub fn player(&self) -> Player{
        self.0.lock().player()
    }
}


