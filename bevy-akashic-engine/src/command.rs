use std::marker::PhantomData;

use auto_delegate::Delegate;

use akashic_rs::prelude::{PointDownHandler, UpdateHandler};
use akashic_rs::prelude::E;
use akashic_rs::trigger::point_move::PointMoveHandler;
use akashic_rs::trigger::point_up::PointUpHandler;

mod append;
mod destroy;
mod audio;

pub mod prelude {
    pub use crate::command::{
        append::*,
        AsBundle,
        audio::prelude::*,
        destroy::*,
    };
}


pub trait AsBundle<B> {
    fn as_bundle(&self) -> B;
}


#[derive(Delegate)]
#[to(E)]
pub struct BoxedEntity<T, B>(T, PhantomData<B>)
    where T: AsBundle<B> + E + 'static;


impl<T, B> AsBundle<B> for BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static
{
    #[inline]
    fn as_bundle(&self) -> B {
        self.0.as_bundle()
    }
}


impl<T, B> BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static
{
    #[inline(always)]
    pub const fn new(akashic_entity: T) -> BoxedEntity<T, B> {
        Self(akashic_entity, PhantomData)
    }
}


unsafe impl<T, B> Sync for BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static
{}

unsafe impl<T, B> Send for BoxedEntity<T, B>
    where T: AsBundle<B> + E + 'static
{}