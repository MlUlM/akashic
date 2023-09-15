use std::cell::OnceCell;

use bevy::prelude::Event;

use akashic_rs::player::Player;
use akashic_rs::prelude::CommonOffset;
use akashic_rs::trigger::PointEventBase;

#[derive(Debug, Event)]
pub(crate) struct PointEventInner<B: PointEventBase> {
    base: B,
    button: OnceCell<u8>,
    event_flags: OnceCell<u8>,
    local: OnceCell<bool>,
    target: OnceCell<Option<akashic_rs::object2d::entity::Entity>>,
    player: OnceCell<Option<Player>>,
    point: OnceCell<CommonOffset>,
    pointer_id: OnceCell<f32>,
}


impl<B: PointEventBase> PointEventBase for PointEventInner<B> {
    #[inline(always)]
    fn button(&self) -> u8 {
        *self.button.get_or_init(|| self.base.button())
    }


    #[inline(always)]
    fn event_flags(&self) -> u8 {
        *self.event_flags.get_or_init(|| self.base.event_flags())
    }


    #[inline(always)]
    fn local(&self) -> bool {
        *self.local.get_or_init(|| self.base.local())
    }


    #[inline(always)]
    fn target(&self) -> Option<akashic_rs::object2d::entity::Entity> {
        self.target.get_or_init(|| self.base.target()).clone()
    }


    #[inline(always)]
    fn player(&self) -> Option<Player> {
        self.player.get_or_init(|| self.base.player()).clone()
    }


    #[inline(always)]
    fn point(&self) -> CommonOffset {
        self.point.get_or_init(|| self.base.point()).clone()
    }


    #[inline(always)]
    fn pointer_id(&self) -> f32 {
        *self.pointer_id.get_or_init(|| self.base.pointer_id())
    }
}


unsafe impl<B: PointEventBase> Send for PointEventInner<B> {}


unsafe impl<B: PointEventBase> Sync for PointEventInner<B> {}