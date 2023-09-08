use bevy::ecs::system::{Command, EntityCommands};
use bevy::prelude::{Bundle, Commands, World};

use akashic_rs::prelude::E;
use akashic_rs::prelude::GAME;

use crate::command::{AsBundle, BoxedEntity};

pub trait AkashicEntityAppend<'w, 's> {
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> + E + 'static) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> AkashicEntityAppend<'w, 's> for Commands<'w, 's> {
    #[inline(always)]
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> + E + 'static) -> EntityCommands<'w, 's, 'a> {
        let bundle = e.as_bundle();
        self.add(Append::new(e));
        self.spawn(bundle)
    }
}


pub struct Append<T, B> where T: AsBundle<B> + E + 'static
{
    e: BoxedEntity<T, B>,
}

impl<T, B> Append<T, B>
    where T:
    AsBundle<B> +
    E +
    'static,
          B: Bundle
{
    #[inline(always)]
    pub const fn new(e: T) -> Append<T, B> {
        Self {
            e: BoxedEntity::new(e)
        }
    }
}

impl<T, B> Command for Append<T, B>
    where T:
    AsBundle<B> +
    E +
    'static,
          B: Bundle
{
    fn apply(self, _: &mut World) {
        GAME.scene().append(&self.e);
    }
}





