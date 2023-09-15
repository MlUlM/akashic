use std::marker::PhantomData;

use auto_delegate::Delegate;

use akashic_rs::entity::Entity;
use akashic_rs::prelude::E;

pub mod audio;

pub mod prelude {
    pub use crate::command::{AsBundle, audio::prelude::*};
}

pub trait AsBundle<B> {
    fn as_bundle(&self) -> B;
}

#[derive(Delegate)]
#[to(E)]
pub struct BoxedAkashicEntity<T, B>(T, PhantomData<B>)
    where
        T: AsBundle<B> + E + 'static;

impl<T, B> AsBundle<B> for BoxedAkashicEntity<T, B>
    where
        T: AsBundle<B> + E + 'static,
{
    #[inline]
    fn as_bundle(&self) -> B {
        self.0.as_bundle()
    }
}

impl<T, B> BoxedAkashicEntity<T, B>
    where
        T: AsBundle<B> + E + 'static,
{
    #[inline(always)]
    pub const fn new(akashic_entity: T) -> BoxedAkashicEntity<T, B> {
        Self(akashic_entity, PhantomData)
    }
}


impl<T, B> Clone for BoxedAkashicEntity<T, B> where T: Clone + AsBundle<B> + E + 'static {
    #[inline(always)]
    fn clone(&self) -> Self {
        BoxedAkashicEntity::new(self.0.clone())
    }
}


#[allow(clippy::from_over_into)]
impl<T, B> Into<akashic_rs::entity::Entity> for BoxedAkashicEntity<T, B> where T: Into<Entity> + AsBundle<B> + E + 'static {
    #[inline(always)]
    fn into(self) -> Entity {
        self.0.into()
    }
}

unsafe impl<T, B> Sync for BoxedAkashicEntity<T, B> where T: AsBundle<B> + E + 'static {}

unsafe impl<T, B> Send for BoxedAkashicEntity<T, B> where T: AsBundle<B> + E + 'static {}


