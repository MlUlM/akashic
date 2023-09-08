use bevy::ecs::system::{Command, EntityCommands};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, World};

use akashic_rs::prelude::{E, UpdateHandler};
use akashic_rs::prelude::GAME;
use akashic_rs::prelude::PointDownHandler;
use akashic_rs::trigger::point_move::PointMoveHandler;
use akashic_rs::trigger::point_up::PointUpHandler;

use crate::command::{AsBundle, BoxedEntity};
use crate::component::AkashicEntityId;
use crate::event::AkashicEventQueue;
use crate::event::point_down::PointDown;
use crate::event::point_move::PointMoveEvent;
use crate::event::point_up::PointUpEvent;
use crate::extensions::AsVec2;

pub trait AkashicEntityAppend<'w, 's> {
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> +
    E +
    UpdateHandler +
    PointUpHandler +
    PointDownHandler +
    PointMoveHandler +
    'static) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> AkashicEntityAppend<'w, 's> for Commands<'w, 's> {
    #[inline(always)]
    fn append<'a, B: Bundle>(&'a mut self, e: impl AsBundle<B> + E + UpdateHandler +
    PointUpHandler +
    PointDownHandler +
    PointMoveHandler +
    'static) -> EntityCommands<'w, 's, 'a> {
        let bundle = e.as_bundle();
        self.add(Append::new(e));
        self.spawn(bundle)
    }
}


pub struct Append<T, B> where T:
AsBundle<B> +
E +
UpdateHandler +
PointUpHandler +
PointDownHandler +
PointMoveHandler +
'static
{
    e: BoxedEntity<T, B>,
}

impl<T, B> Append<T, B>
    where T:
    AsBundle<B> +
    E +
    UpdateHandler +
    PointUpHandler +
    PointDownHandler +
    PointMoveHandler +
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
    UpdateHandler +
    PointUpHandler +
    PointDownHandler +
    PointMoveHandler +
    'static,
          B: Bundle
{
    fn apply(self, world: &mut World) {
        GAME.scene().append(&self.e);

        register_point_down(&self.e, world);
        register_point_up(&self.e, world);
        // register_point_move(&self.e, world);
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


fn register_point_up<T: PointUpHandler + E>(e: &T, world: &mut World) {
    let queue = world.resource::<AkashicEventQueue<PointUpEvent>>().clone();
    let entity_id = Some(AkashicEntityId(e.id()));
    e.on_point_up().add(move |e| {
        queue.push(PointUpEvent {
            entity_id,
            point: Vec2::new(e.point().x(), e.point().y()),
        });
    });
}


fn register_point_move<T: PointMoveHandler + E>(e: &T, world: &mut World) {
    let queue = world.resource::<AkashicEventQueue<PointMoveEvent>>().clone();
    let entity_id = AkashicEntityId(e.id());
    e.on_point_move().add(move |e| {
        queue.push(PointMoveEvent {
            entity_id,
            point: Vec2::new(e.point().x(), e.point().y()),
            start_delta: e.start_delta().as_vec2(),
            prev_delta: e.prev_delta().as_vec2(),
        });
    });
}