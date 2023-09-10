use bevy::app::{App, Update};
use bevy::prelude::{Event, EventReader};
use bevy::reflect::erased_serde::__private::serde;
use bevy::reflect::erased_serde::__private::serde::de::DeserializeOwned;
use bevy::reflect::erased_serde::__private::serde::Serialize;
use std::sync::Arc;

use akashic_rs::event::message::MessageEvent;

use crate::event::AkashicEventQueue;
use crate::plugin::event::read_akashic_event_queue_system;
use akashic_rs::game::GAME;
use akashic_rs::player::Player;
use akashic_rs::prelude::MessageHandler;
use akashic_rs::scene::Scene;

#[derive(Event, Debug, Default)]
pub struct AkashicRaiseEvent<E: Serialize + DeserializeOwned + Event> {
    pub data: E,
    pub player: Option<Player>,
    pub local: Option<bool>,
    pub event_flags: Option<u8>,
}

unsafe impl<E: Serialize + DeserializeOwned + Event> Send for AkashicRaiseEvent<E> {}

unsafe impl<E: Serialize + DeserializeOwned + Event> Sync for AkashicRaiseEvent<E> {}

pub(crate) type RegisterAkashicMessageFn = Arc<dyn Fn(&mut App, &Scene) + Send + Sync>;

pub(crate) fn add_akashic_message_event<E>() -> RegisterAkashicMessageFn
where
    E: Event + DeserializeOwned + serde::Serialize,
{
    Arc::new(|app: &mut App, scene: &Scene| {
        let queue = AkashicEventQueue::<E>::default();
        app.insert_resource(queue.clone());
        app.add_event::<E>();
        app.add_event::<AkashicRaiseEvent<E>>();
        app.add_systems(
            Update,
            (
                raise_event_system::<E>,
                read_akashic_event_queue_system::<E>,
            ),
        );

        scene.on_message().add(move |event| {
            let Some(data) = serde_wasm_bindgen::from_value::<E>(event.data()).ok() else { return; };

            queue.push(data);
        });
    })
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
