use auto_delegate::Delegate;
use bevy::math::Vec2;
use bevy::prelude::Component;
use once_cell::sync::OnceCell;

use akashic::player::Player;
use akashic::prelude::{AkashicEntity, CommonOffset};
use akashic::trigger::{PointDeltaEventBase, PointEventBase};

#[derive(Debug, Clone, Component)]
pub struct PointEventInner<E: PointEventBase + Clone> {
    native_event: E,
    button: OnceCell<u8>,
    event_flags: OnceCell<u8>,
    local: OnceCell<bool>,
    target: OnceCell<Option<AkashicEntity>>,
    player: OnceCell<Option<Player>>,
    player_id: OnceCell<String>,
    point: OnceCell<CommonOffset>,
    pointer_id: OnceCell<f32>,
}


impl<E: PointEventBase + Clone> PointEventInner<E> {
    pub(crate) fn new(native_event: E) -> PointEventInner<E> {
        Self {
            native_event,
            button: Default::default(),
            event_flags: Default::default(),
            local: Default::default(),
            player: Default::default(),
            target: Default::default(),
            player_id: Default::default(),
            point: Default::default(),
            pointer_id: Default::default(),
        }
    }
}


impl<B: PointEventBase + Clone> PointEventBase for PointEventInner<B> {
    #[inline(always)]
    fn button(&self) -> u8 {
        *self.button.get_or_init(|| self.native_event.button())
    }


    #[inline(always)]
    fn event_flags(&self) -> u8 {
        *self.event_flags.get_or_init(|| self.native_event.event_flags())
    }

    #[inline(always)]
    fn local(&self) -> bool {
        *self.local.get_or_init(|| self.native_event.local())
    }

    #[inline(always)]
    fn target(&self) -> Option<AkashicEntity> {
        self.target.get_or_init(|| self.native_event.target()).clone()
    }

    #[inline(always)]
    fn player(&self) -> Option<Player> {
        self.player.get_or_init(|| self.native_event.player()).clone()
    }

    #[inline(always)]
    fn point(&self) -> CommonOffset {
        self.point.get_or_init(|| self.native_event.point()).clone()
    }


    #[inline(always)]
    fn pointer_id(&self) -> f32 {
        *self.pointer_id.get_or_init(|| self.native_event.pointer_id())
    }
}


unsafe impl<B: PointEventBase + Clone> Send for PointEventInner<B> {}


unsafe impl<B: PointEventBase + Clone> Sync for PointEventInner<B> {}


#[derive(Debug, Component, Clone, Delegate)]
pub struct PointDeltaEventInner<E: PointDeltaEventBase + Clone> {
    #[to(AkashicPointEventBase, PointEventBase)]
    base: PointEventInner<E>,
    native_event: E,
    start_delta: OnceCell<Vec2>,
    prev_delta: OnceCell<Vec2>,
}


impl<E: PointDeltaEventBase + Clone> PointDeltaEventInner<E> {
    pub(crate) fn new(native_event: E) -> PointDeltaEventInner<E> {
        Self {
            base: PointEventInner::new(native_event.clone()),
            native_event,
            start_delta: Default::default(),
            prev_delta: Default::default(),
        }
    }
}


impl<B: PointDeltaEventBase + Clone> PointDeltaEventInner<B> {
    #[inline(always)]
    pub fn start_delta(&self) -> Vec2 {
        *self.start_delta.get_or_init(|| {
            let point = self.native_event.start_delta();
            Vec2::new(point.x(), point.y())
        })
    }


    #[inline(always)]
    pub fn prev_delta(&self) -> Vec2 {
        *self.prev_delta.get_or_init(|| {
            let point = self.native_event.prev_delta();
            Vec2::new(point.x(), point.y())
        })
    }
}


unsafe impl<B: PointDeltaEventBase + Clone> Send for PointDeltaEventInner<B> {}

unsafe impl<B: PointDeltaEventBase + Clone> Sync for PointDeltaEventInner<B> {}
