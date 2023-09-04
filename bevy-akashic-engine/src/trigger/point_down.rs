use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use bevy::math::Vec2;
use bevy::prelude::{Event, Resource};

use crate::component::AkashicEntityId;

#[derive(Clone, Debug, Event)]
pub struct ScenePointDown {
    pub point: Vec2,
}

#[derive(Clone, Debug, Event)]
pub struct PointDown {
    pub entity_id: AkashicEntityId,
    pub point: Vec2,
}


#[derive(Resource,Clone)]
pub(crate) struct AkashicEventQueue<T>(pub Arc<Mutex<VecDeque<T>>>);


impl<T: Event> AkashicEventQueue<T> {
    #[inline(always)]
    pub fn push(&self, event: T) {
        self.0.lock().unwrap().push_back(event);
    }


    #[inline(always)]
    pub fn pop_front(&self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }
}



impl<T> Default for AkashicEventQueue<T>{
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }
}