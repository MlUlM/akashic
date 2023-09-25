use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use bevy::prelude::{Resource};

pub mod point_down;
pub mod point_up;
pub mod point_move;
pub(crate) mod event_inner;
pub mod message;
pub mod join;





#[derive(Resource)]
pub struct AkashicEventQueue<T>(pub Arc<Mutex<VecDeque<T>>>);


impl<T> Clone for AkashicEventQueue<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}


impl<T> AkashicEventQueue<T> {
    #[inline(always)]
    pub fn push(&self, event: T) {
        self.0.lock().unwrap().push_back(event);
    }


    #[inline(always)]
    pub fn pop_front(&self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }


    #[inline(always)]
    pub fn clear(&self){
        self.0.lock().unwrap().clear();
    }
}


impl<T> Default for AkashicEventQueue<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }
}