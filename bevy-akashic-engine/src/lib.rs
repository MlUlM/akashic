use std::sync::{Arc, Mutex, MutexGuard};

mod command;

mod plugin;
mod trigger;
mod component;
mod asset;

pub mod prelude {
    pub use crate::command::prelude::*;
    pub use crate::plugin::*;
    pub use crate::asset::*;
    pub use akashic_rs::prelude::*;
    pub use crate::trigger::*;
    pub use crate::component::*;
}


#[derive(Clone)]
pub(crate) struct SharedObject<T>(Arc<Mutex<T>>);


impl<T> SharedObject<T> {
    #[inline(always)]
    pub fn new(v: T) -> SharedObject<T> {
        Self(Arc::new(Mutex::new(v)))
    }


    #[inline(always)]
    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

unsafe impl<T> Send for SharedObject<T> {}

unsafe impl<T> Sync for SharedObject<T> {}

