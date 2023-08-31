use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use bevy::math::Vec2;
use bevy::prelude::{Event, Resource};
use crate::component::AkashicEntityId;

#[derive(Copy, Clone, Debug, Event)]
pub struct PointDown {
    pub entity_id: AkashicEntityId,
    pub point: Vec2,
}


#[derive(Resource, Default, Clone)]
pub(crate) struct PointDownQueue(pub Arc<Mutex<VecDeque<PointDown>>>);


impl PointDownQueue {
    #[inline(always)]
    pub fn push(&self, event: PointDown) {
        self.0.lock().unwrap().push_back(event);
    }


    #[inline(always)]
    pub fn pop_front(&self) -> Option<PointDown> {
        self.0.lock().unwrap().pop_front()
    }
}