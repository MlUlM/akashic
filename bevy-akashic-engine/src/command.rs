use std::marker::PhantomData;

use auto_delegate::Delegate;
use bevy::prelude::Bundle;
use wasm_bindgen::JsValue;

use akashic_rs::prelude::{PointDownEvent, PointDownHandler, Trigger, UpdateHandler, Void};
use akashic_rs::prelude::E;

mod append;
mod destroy;
mod audio;

pub mod prelude{
    pub use crate::command::{
        AsBundle,
        append::*,
        destroy::*,
        audio::prelude::*,
    };
}


pub trait AsBundle<B> {
    fn as_bundle(&self) -> B;
}


#[derive(Delegate)]
pub struct BoxedEntity<T, B>(T, PhantomData<B>);

impl<T, B> BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    #[inline]
    pub const fn new(e: T) -> BoxedEntity<T, B> {
        Self(e, PhantomData)
    }
}

unsafe impl<T, B> Send for BoxedEntity<T, B> {}

unsafe impl<T, B> Sync for BoxedEntity<T, B> {}

impl<T, B> UpdateHandler for BoxedEntity<T, B>
    where T: UpdateHandler + 'static,
          B: Bundle
{
    #[inline(always)]
    fn on_update(&self) -> Trigger<Void> {
        self.0.on_update()
    }
}


impl<T, B> PointDownHandler for BoxedEntity<T, B>
    where T: PointDownHandler + 'static,
          B: Bundle
{
    #[inline(always)]
    fn on_point_down(&self) -> Trigger<PointDownEvent> {
        self.0.on_point_down()
    }
}


impl<T, B> E for BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    #[inline(always)]
    fn id(&self) -> usize {
        self.0.id()
    }


    #[inline(always)]
    fn as_js_value(&self) -> JsValue {
        self.0.as_js_value()
    }
}


impl<T, B> AsBundle<B> for BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    #[inline(always)]
    fn as_bundle(&self) -> B {
        self.0.as_bundle()
    }
}
