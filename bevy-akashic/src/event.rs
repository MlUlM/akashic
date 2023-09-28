use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use bevy::prelude::{Deref, Resource};


pub mod message;
pub mod join;
pub mod point;


#[derive(Resource, Deref)]
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
    pub fn is_empty(&self) -> bool {
        self.0.lock().unwrap().is_empty()
    }

    #[inline(always)]
    pub fn clear(&self) {
        self.0.lock().unwrap().clear();
    }
}


impl<T> Default for AkashicEventQueue<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }
}