use bevy::ecs::system::Command;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, World};

use akashic_rs::entity::E;
use akashic_rs::game::GAME;

use crate::command::{AsBundle, BoxedEntity};
use crate::component::AkashicEntityId;
use crate::trigger::point_down::{PointDown, PointDownQueue};

pub trait AkashicCommandEx {
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + E + 'static);
}

impl<'w, 's> AkashicCommandEx for Commands<'w, 's> {
    #[inline(always)]
    fn append<B: Bundle>(&mut self, e: impl AsBundle<B> + E + 'static) {
        self.add(Append::new(e));
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
    where T: AsBundle<B> + E + 'static,
          B: Bundle
{
    fn apply(self, world: &mut World) {
        let bundle = self.e.as_bundle();
        register_point_down(&self.e, world);

        GAME.scene().append(&self.e);
        world.spawn(bundle);
    }
}


fn register_point_down(e: &impl E, world: &mut World) {
    let point_down_queue = world.resource::<PointDownQueue>().clone();
    let entity_id = AkashicEntityId(e.id());

    e.on_point_down().add(move |e| {
        point_down_queue.push(PointDown {
            entity_id,
            point: Vec2::new(e.point().x(), e.point().y()),
        });
    });
}