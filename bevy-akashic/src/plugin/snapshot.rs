use std::io;
use std::io::Write;

use bevy::app::{App, Last, Plugin, PreStartup};
use bevy::log::info;
use bevy::prelude::{Commands, Deref, DerefMut, IntoSystemConfigs, NextState, Reflect, Res, ResMut, Resource, States, Time, TimerMode, World};
use bevy::time::Timer;
use bevy_save::{SavePlugins, WorldSaveableExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic::event::message::MessageEvent;
use akashic::game::GAME;

use crate::plugin::system_set::AkashicSystemSet;

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
        let len = buf.len();

        self.0.extend_from_slice(buf);
        Ok(len)
    }


    fn flush(&mut self) -> io::Result<()> {
        GAME.raise_event(MessageEvent::from_serde(&mut SnapshotData {
            snap: true,
            data: Box::from(self.0.clone().into_boxed_slice()),
        }, None, None, None));
        Ok(())
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SnapshotTimer(Timer);


#[derive(Serialize, Deserialize)]
pub struct SnapshotData {
    pub snap: bool,
    pub data: Box<[u8]>,
}

pub struct SnapshotPlugin;

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SnapshotTimer(Timer::from_seconds(5., TimerMode::Repeating)))
            .add_state::<AkashicSetupState>()
            .add_plugins(SavePlugins)
            .add_systems(PreStartup, setup)
            .add_systems(Last, request_save_snapshot_if_need.in_set(AkashicSystemSet::RequestSaveSnapShot));
    }
}


fn setup(
    mut commands: Commands,
    mut state: ResMut<NextState<AkashicSetupState>>,
) {
    let snapshot = snapshot();
    info!("shapshot = {snapshot:?}");
    if snapshot.is_empty() {
        state.set(AkashicSetupState::Init);
    } else {
        commands.add(move |world: &mut World| {
            info!("{:?}",  json!(snapshot));

            world.deserialize(&mut serde_json::Deserializer::from_slice(&snapshot)).unwrap();
        });
        state.set(AkashicSetupState::WithSnapshot);
    }
}


fn request_save_snapshot_if_need(
    mut commands: Commands,
    mut timer: ResMut<SnapshotTimer>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).just_finished() {
        commands.add(|world: &mut World| {
            // let file = File::create("test.json").expect("Could not open file for serialization");
            // let mut s = serde_json::Serializer::pretty(file);
            let mut s = serde_json::Serializer::new(SnapshotIo::default());

            world.serialize(&mut s).unwrap();
            s.into_inner().flush();
        });
    }
}


#[wasm_bindgen(js_namespace = g)]
extern {
    fn snapshot() -> Box<[u8]>;
}