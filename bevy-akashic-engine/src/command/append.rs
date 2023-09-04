use bevy::ecs::system::{Command, EntityCommands};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, World};

use akashic_rs::prelude::E;
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::PointDownHandler;

use crate::command::{AsBundle, BoxedEntity};
use crate::component::AkashicEntityId;
use crate::trigger::point_down::{AkashicEventQueue, PointDown};

pub trait AkashicEntityAppend<'w, 's> {
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> + E + PointDownHandler + 'static) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> AkashicEntityAppend<'w, 's> for Commands<'w, 's> {
    #[inline(always)]
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> + E + PointDownHandler + 'static) -> EntityCommands<'w, 's, 'a> {
        let bundle = e.as_bundle();
        self.add(Append::new(e));
        self.spawn(bundle)
    }
}


pub struct Append<T, B> {
    e: BoxedEntity<T, B>,
}

impl<T, B> Append<T, B>
    where T: AsBundle<B> + E + 'static,
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
    where T: AsBundle<B> + E + PointDownHandler + 'static,
          B: Bundle
{
    fn apply(self, world: &mut World) {
        GAME.scene().append(&self.e);
        register_point_down(&self.e, world);
    }
}


fn register_point_down<T: PointDownHandler + E>(e: &T, world: &mut World) {
    let point_down_queue = world.resource::<AkashicEventQueue<PointDown>>().clone();
    let entity_id = AkashicEntityId(e.id());

    e.on_point_down().add(move |e| {
        point_down_queue.push(PointDown {
            entity_id,
            point: Vec2::new(e.point().x(), e.point().y()),
        });
    });
}