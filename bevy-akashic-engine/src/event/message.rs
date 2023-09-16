use bevy::app::{App, Update};
use bevy::prelude::{Event, EventReader, EventWriter, Res};
use bevy::reflect::erased_serde::__private::serde;
use bevy::reflect::erased_serde::__private::serde::de::DeserializeOwned;
use bevy::reflect::erased_serde::__private::serde::Serialize;

use akashic_rs::event::message::MessageEvent;
use akashic_rs::game::GAME;
use akashic_rs::player::Player;
use akashic_rs::prelude::MessageHandler;

use crate::event::AkashicEventQueue;

#[derive(Event, Debug, Default)]
pub struct AkashicRaiseEvent<E: Serialize + DeserializeOwned + Event> {
    pub data: E,
    pub player: Option<Player>,
    pub local: Option<bool>,
    pub event_flags: Option<u8>,
}

unsafe impl<E: Serialize + DeserializeOwned + Event> Send for AkashicRaiseEvent<E> {}

unsafe impl<E: Serialize + DeserializeOwned + Event> Sync for AkashicRaiseEvent<E> {}


pub trait AddMessageEvent {
    fn add_message_event<E>(&mut self) -> &mut Self
        where E: Event + DeserializeOwned + serde::Serialize;
}


impl AddMessageEvent for App {
    fn add_message_event<E>(&mut self) -> &mut Self where E: Event + DeserializeOwned + Serialize {
        let queue = AkashicEventQueue::<E>::default();
        self.insert_resource(queue.clone());
        self.add_event::<E>();
        self.add_event::<AkashicRaiseEvent<E>>();
        self.add_systems(
            Update,
            (
                raise_event_system::<E>,
                read_akashic_event_queue_system::<E>,
            ),
        );

        GAME
            .scene()
            .on_message()
            .add(move |event| {
                let Some(data) = serde_wasm_bindgen::from_value::<E>(event.data()).ok() else { return; };
                queue.push(data);
            });

        self
    }
}


fn raise_event_system<E>(mut er: EventReader<AkashicRaiseEvent<E>>)
    where
        E: Event + serde::Serialize + DeserializeOwned,
{
    for event in er.iter() {
        GAME.raise_event(MessageEvent::from_serde(
            &event.data,
            event.player.clone(),
            event.local,
            event.event_flags,
        ));
    }
}


fn read_akashic_event_queue_system<T: Event>(
    mut ew: EventWriter<T>,
    queue: Res<AkashicEventQueue<T>>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(event);
    }
}