use std::sync::{Arc, Mutex, MutexGuard};
use wasm_bindgen::JsValue;
use akashic_rs::game::GAME;
use akashic_rs::prelude::{Scene, SceneParameterObject};

mod command;

mod plugin;
pub mod event;
mod component;
mod asset;
mod extensions;

pub mod akashic {
    pub use akashic_rs::*;
}

pub mod prelude {
    pub use crate::command::prelude::*;
    pub use crate::plugin::*;
    pub use crate::asset::*;

    pub use akashic_rs::prelude::*;
    pub use crate::event::*;
    pub use crate::component::*;
}


#[derive(Clone, Debug)]
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


pub fn register_scene(param: SceneParameterObject) {
    GAME.push_scene(
        Scene::new(param),
        JsValue::UNDEFINED,
    );
}