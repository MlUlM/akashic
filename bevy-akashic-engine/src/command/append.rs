use bevy::ecs::system::{Command, EntityCommands};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{Bundle, Commands, Entity, World};
use akashic_rs::entity::AppendEntity;

use akashic_rs::prelude::E;
use akashic_rs::prelude::GAME;

use crate::command::{AsBundle, BoxedAkashicEntity};
use crate::plugin::akashic_entity_map::AkashicEntityMap;


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

struct Append<T, B> where T: AsBundle<B> + E + 'static
{
    e: BoxedAkashicEntity<T, B>,
}

impl<T, B> Append<T, B>
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    #[inline(always)]
    pub const fn new(e: T) -> Append<T, B> {
        Self {
            e: BoxedAkashicEntity::new(e)
        }
    }
}


impl<T, B> Command for Append<T, B>
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    #[inline(always)]
    fn apply(self, _: &mut World) {
        GAME.scene().append(&self.e);
    }
}


pub trait AkashicEntityChildAppend<'w, 's, 'a> {
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + E + Into<akashic_rs::entity::Entity> + 'static) -> &mut EntityCommands<'w, 's, 'a>;
}


impl<'w, 's, 'a> AkashicEntityChildAppend<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {
    #[inline(always)]
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + E + Into<akashic_rs::entity::Entity> + 'static) -> &mut EntityCommands<'w, 's, 'a> {
        self.with_children(|builder| {
            builder.spawn(e.as_bundle());
        });

        let e = BoxedAkashicEntity::new(e);
        self.add(move |parent_entity: Entity, world: &mut World| {
            let Some(parent_akashic_entity) = world
                .resource::<AkashicEntityMap>()
                .get(&parent_entity) else { return; };

            parent_akashic_entity.append(e)
        })
    }
}


pub trait AkashicEntityChildBuilderAppend<'w, 's, 'a> {
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + Clone + E + Into<akashic_rs::entity::Entity> + 'static) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> AkashicEntityChildBuilderAppend<'w, 's, 'a> for ChildBuilder<'w, 's, 'a> {
    #[inline(always)]
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + Clone + E + Into<akashic_rs::entity::Entity> + 'static) -> EntityCommands<'w, 's, '_> {
        let parent_entity = self.parent_entity();

        let e = BoxedAkashicEntity::new(e);
        let e2 = e.clone();
        self.add_command(move |world: &mut World| {
            let Some(parent_akashic_entity) = world
                .resource::<AkashicEntityMap>()
                .get(&parent_entity) else { return; };

            parent_akashic_entity.append(e2)
        });

        self.spawn(e.as_bundle())
    }
}

