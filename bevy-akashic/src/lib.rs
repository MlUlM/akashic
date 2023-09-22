#![allow(clippy::type_complexity)]

use std::sync::{Arc, Mutex, MutexGuard};

use bevy::prelude::Deref;
use wasm_bindgen::JsValue;

use akashic::game::GAME;
use akashic::prelude::{Scene, SceneParameterObject};

pub mod command;

pub mod plugin;
pub mod event;
pub mod component;
pub mod extensions;
pub mod resource;
pub mod run_criteria;

pub use akashic::*;

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


pub fn register_scene(param: SceneParameterObject) {
    GAME.push_scene(
        Scene::new(param),
        JsValue::UNDEFINED,
    );
}