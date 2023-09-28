use bevy_akashic::event::point::up::OnPointUp;
use bevy_akashic::plugin::event::AkashicPointUpEvent;

use crate::input::akashic_pointer::macros::akashic_pointer_plugin;

akashic_pointer_plugin!(AkashicPointUpPatchPlugin, AkashicPointUpEvent, OnPointUp);


