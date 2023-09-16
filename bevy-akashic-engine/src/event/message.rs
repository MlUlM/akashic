use bevy::app::{App, Update};
use bevy::prelude::{EventWriter, Res};
use bevy::reflect::erased_serde::__private::serde;
use bevy::reflect::erased_serde::__private::serde::de::DeserializeOwned;
use bevy::reflect::erased_serde::__private::serde::Serialize;

use akashic_rs::game::GAME;
use akashic_rs::prelude::MessageHandler;

use crate::event::AkashicEventQueue;
use crate::event::message::raise_event::RaiseEvent;

pub mod raise_event;
pub mod request_raise_event;

pub trait AddMessageEvent {
    fn add_message_event<E>(&mut self) -> &mut Self
        where E: DeserializeOwned + serde::Serialize + 'static;
}


impl AddMessageEvent for App {
    fn add_message_event<E>(&mut self) -> &mut Self
        where E: DeserializeOwned + Serialize + 'static
    {
        let queue = AkashicEventQueue::<RaiseEvent<E>>::default();
        self.insert_resource(queue.clone());
        self.add_event::<RaiseEvent<E>>();
        self.add_systems(
            Update,
            (
                read_akashic_event_queue_system::<E>,
            ),
        );

        GAME
            .scene()
            .on_message()
            .add(move |event| {
                let Some(data) = serde_wasm_bindgen::from_value::<E>(event.data()).ok() else { return; };
                queue.push(RaiseEvent::new(event, data));
            });

        self
    }
}


fn read_akashic_event_queue_system<E: Serialize + DeserializeOwned + 'static>(
    mut ew: EventWriter<RaiseEvent<E>>,
    queue: Res<AkashicEventQueue<RaiseEvent<E>>>,
) {
    while let Some(event) = queue.pop_front() {
        ew.send(event);
    }
}