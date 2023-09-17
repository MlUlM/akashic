use std::fmt::{Debug, Formatter};

use bevy::prelude::Event;
use once_cell::sync::OnceCell;

use akashic_rs::event::message::MessageEvent;
use akashic_rs::player::Player;

#[derive(Event)]
pub struct RaiseEvent<E> {
    pub data: E,
    native_event: MessageEvent,
    player: OnceCell<Option<Player>>,
    local: OnceCell<bool>,
    event_flags: OnceCell<u8>,
}


impl<E> RaiseEvent<E> {
    pub(crate) fn new(native_event: MessageEvent, data: E) -> RaiseEvent<E> {
        Self {
            native_event,
            data,
            player: Default::default(),
            local: Default::default(),
            event_flags: Default::default(),
        }
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
    pub fn event_flags(&self) -> u8 {
        *self.event_flags.get_or_init(|| self.native_event.event_flags())
    }
}


impl<E: Debug> Debug for RaiseEvent<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("RaiseEvent")
            .field("data", &self.data)
            .field("local", &self.local())
            .field("player", &self.player())
            .field("event_flags", &self.event_flags())
            .finish()
    }
}


unsafe impl<E> Send for RaiseEvent<E> {}

unsafe impl<E> Sync for RaiseEvent<E> {}