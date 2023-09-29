use bevy::ecs::system::SystemParam;
use bevy::prelude::{Event, FromWorld};
use serde::de::DeserializeOwned;
use serde::Serialize;
use akashic::event::message::MessageEvent;
use akashic::game::GAME;
use akashic::player::Player;

#[derive(Event, Debug, Default)]
pub struct RequestRaiseEvent<E: Serialize + DeserializeOwned + 'static> {
    pub data: E,
    pub player: Option<Player>,
    pub local: Option<bool>,
    pub event_flags: Option<u8>,
}


impl<E> RequestRaiseEvent<E>
    where E: Serialize + DeserializeOwned + 'static
{
    #[inline]
    pub fn new(data: E) -> RequestRaiseEvent<E> {
        Self {
            data,
            player: None,
            local: None,
            event_flags: None,
        }
    }


    #[inline]
    pub fn with_player(&mut self, player: Player) -> &mut RequestRaiseEvent<E> {
        self.player = Some(player);
        self
    }


    #[inline]
    pub fn with_local(&mut self, local: bool) -> &mut RequestRaiseEvent<E> {
        self.local = Some(local);
        self
    }


    #[inline]
    pub fn with_event_flags(&mut self, event_flags: u8) -> &mut RequestRaiseEvent<E> {
        self.event_flags = Some(event_flags);
        self
    }
}


unsafe impl<E: Serialize + DeserializeOwned + Event> Send for RequestRaiseEvent<E> {}

unsafe impl<E: Serialize + DeserializeOwned + Event> Sync for RequestRaiseEvent<E> {}


#[derive(SystemParam, Debug, Copy, Clone)]
pub struct RaiseEventRequester;


impl RaiseEventRequester {
    #[inline]
    pub fn raise_only_data<E>(&self, data: E)
        where E: Serialize + DeserializeOwned + 'static
    {
        self.raise(RequestRaiseEvent::new(data))
    }


    #[inline]
    pub fn raise<E>(&self, request: RequestRaiseEvent<E>)
        where E: Serialize + DeserializeOwned + 'static
    {
        GAME.raise_event(MessageEvent::from_serde(
            &request.data,
            request.player.clone(),
            request.local,
            request.event_flags,
        ));
    }
}