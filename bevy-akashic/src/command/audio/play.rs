use bevy::ecs::system::Command;
use bevy::prelude::{Commands, World};
use akashic::asset::audio::AudioAsset;

use crate::command::audio::SharedAudioAsset;
use crate::SharedObject;

pub trait PlayAudio {
    /// 音楽の再生処理を実行するコマンドをキューにエントリーします。
    ///
    ///
    /// ## Notes
    ///
    /// 音楽の再生はこのメソッドの呼び出し時ではなく各コマンドの評価が行われるタイミングです。
    fn play_audio(&mut self, audio_asset: AudioAsset);
}


impl<'w, 's> PlayAudio for Commands<'w, 's> {
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