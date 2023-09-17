use bevy::app::App;
use bevy::prelude::Plugin;
use crate::resource::random::{AkashicLocalRandomGenerator, AkashicRandomGenerator};

pub struct AkashicRandomPlugin;


impl Plugin for AkashicRandomPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AkashicRandomGenerator>()
            .init_resource::<AkashicLocalRandomGenerator>();
    }
}