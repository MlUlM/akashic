use std::sync::{Arc, Mutex, MutexGuard};
use bevy::prelude::Deref;
use wasm_bindgen::JsValue;
use akashic_rs::game::GAME;
use akashic_rs::prelude::{Scene, SceneParameterObject};

pub mod command;

pub mod plugin;
pub mod event;
pub mod component;
pub mod extensions;
pub mod resource;
pub mod run_criteria;

pub mod akashic {
    pub use akashic_rs::*;
}

pub mod prelude {
    pub use crate::command::prelude::*;
    pub use crate::plugin::prelude::*;
    pub use akashic_rs::prelude::*;
    pub use crate::event::*;
    pub use crate::component::*;
}


#[derive(Debug, Default, Deref)]
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

impl<T> Clone for SharedObject<T>{
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