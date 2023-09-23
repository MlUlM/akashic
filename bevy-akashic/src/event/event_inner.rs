use bevy::prelude::{Component, Vec3};
use once_cell::sync::OnceCell;

use akashic::player::Player;
use akashic::trigger::{PointDeltaEventBase, PointEventBase};


#[derive(Debug, Component)]
pub struct PointEventInner<E: PointEventBase> {
    native_event: E,
    button: OnceCell<u8>,
    event_flags: OnceCell<u8>,
    local: OnceCell<bool>,
    player: OnceCell<Option<Player>>,
    player_id: OnceCell<String>,
    point: OnceCell<Vec3>,
    pointer_id: OnceCell<f32>,
    half_game_width: f32,
    half_game_height: f32,
}


impl<E: PointEventBase> PointEventInner<E> {
    pub(crate) fn new(
        native_event: E,
        half_game_width: f32,
        half_game_height: f32,
    ) -> PointEventInner<E> {
        Self {
            native_event,
            button: Default::default(),
            event_flags: Default::default(),
            local: Default::default(),
            player: Default::default(),
            player_id: Default::default(),
            point: Default::default(),
            pointer_id: Default::default(),
            half_game_width,
            half_game_height,
        }
    }
}


macro_rules! event_base_methods {
    () => {
        #[inline(always)]
        pub fn button(&self) -> u8 {
            *self.button.get_or_init(|| self.native_event.button())
        }


        #[inline(always)]
        pub fn event_flags(&self) -> u8 {
            *self.event_flags.get_or_init(|| self.native_event.event_flags())
        }


        #[inline(always)]
        pub fn local(&self) -> bool {
            *self.local.get_or_init(|| self.native_event.local())
        }


        #[inline(always)]
        pub fn player(&self) -> &Option<Player> {
            self.player.get_or_init(|| self.native_event.player())
        }


         #[inline(always)]
        pub fn player_id(&self) -> String {
            self.player_id.get_or_init(|| self.player().as_ref().unwrap().id().unwrap()).clone()
        }


        #[inline(always)]
        pub fn point(&self) -> Vec3 {
            *self.point.get_or_init(|| {
                let point = self.native_event.point();
                Vec3::new(point.x() - self.half_game_width, self.half_game_height - point.y(), 0.)
            })
        }


        #[inline(always)]
        pub fn pointer_id(&self) -> f32 {
            *self.pointer_id.get_or_init(|| self.native_event.pointer_id())
        }
    };
}


impl<B: PointEventBase> PointEventInner<B> {
    event_base_methods!();
}


unsafe impl<B: PointEventBase> Send for PointEventInner<B> {}


unsafe impl<B: PointEventBase> Sync for PointEventInner<B> {}


#[derive(Debug, Component)]
pub struct PointDeltaEventInner<E: PointDeltaEventBase> {
    native_event: E,
    button: OnceCell<u8>,
    event_flags: OnceCell<u8>,
    local: OnceCell<bool>,
    player: OnceCell<Option<Player>>,
    player_id: OnceCell<String>,
    point: OnceCell<Vec3>,
    pointer_id: OnceCell<f32>,
    start_delta: OnceCell<Vec3>,
    prev_delta: OnceCell<Vec3>,
    half_game_width: f32,
    half_game_height: f32,
}


impl<E: PointDeltaEventBase> PointDeltaEventInner<E> {
    pub(crate) fn new(
        native_event: E,
        half_game_width: f32,
        half_game_height: f32,
    ) -> PointDeltaEventInner<E> {
        Self {
            native_event,
            button: Default::default(),
            event_flags: Default::default(),
            local: Default::default(),
            player: Default::default(),
            player_id: Default::default(),
            point: Default::default(),
            pointer_id: Default::default(),
            start_delta: Default::default(),
            prev_delta: Default::default(),
            half_game_width,
            half_game_height,
        }
    }
}


impl<B: PointDeltaEventBase> PointDeltaEventInner<B> {
    event_base_methods!();

    #[inline(always)]
    pub fn start_delta(&self) -> Vec3 {
        *self.start_delta.get_or_init(|| {
            let point = self.native_event.point();
            Vec3::new(point.x(), point.y(), 0.)
        })
    }


    #[inline(always)]
    pub fn prev_delta(&self) -> Vec3 {
        *self.prev_delta.get_or_init(|| {
            let point = self.native_event.point();
            Vec3::new(point.x(), point.y(), 0.)
        })
    }
}


unsafe impl<B: PointDeltaEventBase> Send for PointDeltaEventInner<B> {}

unsafe impl<B: PointDeltaEventBase> Sync for PointDeltaEventInner<B> {}
