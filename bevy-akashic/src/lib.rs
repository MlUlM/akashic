#![allow(clippy::type_complexity)]

use std::sync::{Arc, Mutex, MutexGuard};

use bevy::prelude::Deref;

pub use akashic::*;

pub mod command;

pub mod plugin;
pub mod event;
pub mod component;
pub mod extensions;
pub mod resource;
pub mod run_criteria;

pub mod prelude {
    pub use crate::command::prelude::*;
    pub use crate::component::*;
    pub use crate::event::*;
    pub use crate::plugin::prelude::*;
}


#[derive(Debug, Default, Deref)]
pub(crate) struct SharedObject<T: 'static>(Arc<Mutex<T>>);


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

impl<T> Clone for SharedObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}




#[macro_export]
macro_rules! unsafe_impl_all_synchronization {
    ($struct_name: ident) => {
        unsafe impl Send for $struct_name{}
        
        unsafe impl Sync for $struct_name{}
    };
}