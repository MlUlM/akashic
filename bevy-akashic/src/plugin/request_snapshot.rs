use std::io;

use bevy::app::{App, Last, PreStartup};
use bevy::prelude::{Commands, Deref, DerefMut, IntoSystemConfigs, NextState, ResMut, Resource, States, World};
use bevy_save::{AppLoader, AppSaver, Loader, Reader, SavePlugin, Saver, WorldSaveableExt, Writer};
use bevy_save::erased_serde::IntoSerializer;
use bevy_save::prelude::IntoDeserializer;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic::event::message::MessageEvent;
use akashic::game::GAME;

use crate::event::message::request_raise_event::RaiseEventRequester;
use crate::plugin::system_set::AkashicSystemSet;
use crate::unsafe_impl_all_synchronization;

pub trait Requestable: Sized {
    fn from_world(world: &mut World) -> Option<Self>;
}


#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SnapshotResource<T>(T);
unsafe_impl_all_synchronization!(SnapshotResource, T);

#[derive(States, Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
pub enum AkashicSetupState {
    #[default]
    Undefined,

    Init,

    WithSnapshot,
}


#[derive(Default)]
struct SnapshotIo(Vec<u8>);

impl io::Write for SnapshotIo {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        GAME.raise_event(MessageEvent::from_serde(&mut self.0, None, None, None));

        Ok(())
    }
}




#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, Default)]
struct SnapNone;


pub trait SnapshotSystem {
    fn add_snapshot_systems<T>(&mut self) -> &mut Self
        where T: 'static + serde::de::DeserializeOwned + Requestable + Serialize;
}


struct JSONSaver;

impl Saver for JSONSaver {
    fn serializer<'w>(&self, _: Writer<'w>) -> IntoSerializer<'w> {
        IntoSerializer::erase(serde_json::Serializer::new(SnapshotIo::default()))
    }
}

struct JSONLoader(&'static [u8]);

impl Loader for JSONLoader {
    fn deserializer<'r, 'de>(&self, _: Reader<'r>) -> IntoDeserializer<'r, 'de> {
        IntoDeserializer::erase(serde_json::Deserializer::from_slice(self.0))
    }
}


impl SnapshotSystem for App {
    fn add_snapshot_systems<T>(&mut self) -> &mut Self
        where T: 'static + serde::de::DeserializeOwned + Requestable + Serialize {
        use crate::event::message::AddMessageEvent;
        static BUFF : Lazy<Box<[u8]>> = Lazy::new(||{
            snapshot()
        });

        self
            .add_plugins(SavePlugin)
            .insert_resource(AppSaver::new(JSONSaver))
            .insert_resource(AppLoader::new(JSONLoader(&BUFF)))
            .add_systems(Last, request_save_snapshot_if_need::<T>.in_set(AkashicSystemSet::RequestSaveSnapShot))
    }
}

impl Requestable for SnapNone {
    fn from_world(_: &mut World) -> Option<Self> {

        None
    }
}




fn request_save_snapshot_if_need<T: Requestable + DeserializeOwned + Serialize + 'static>(
    mut commands: Commands,
) {
    commands.add(|world: &mut World| {
        world.save("data").unwrap();
    });
}


#[wasm_bindgen(js_namespace = g)]
extern {
    fn snapshot() -> Box<[u8]>;
}