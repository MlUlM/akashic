use bevy::ecs::system::Command;
use bevy::prelude::{Commands, World};
use akashic_rs::prelude::AudioAsset;

use crate::command::audio::SharedAudioAsset;
use crate::SharedObject;

pub trait PlayAudio {
    fn play_audio(&mut self, audio_asset: AudioAsset);
}


impl<'w, 's> PlayAudio for Commands<'w, 's>{
    fn play_audio(&mut self, audio_asset: AudioAsset) {
        self.add(PlayAudioCommand(SharedObject::new(audio_asset)))
    }
}



struct PlayAudioCommand(SharedAudioAsset);


impl Command for PlayAudioCommand {
    fn apply(self, _: &mut World) {
        self.0.lock().play();
    }
}