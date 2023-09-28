use bevy_akashic::event::point::down::OnPointDown;
use bevy_akashic::plugin::event::AkashicPointDownEvent;

use crate::input::akashic_pointer::macros::akashic_pointer_plugin;

akashic_pointer_plugin!(AkashicPointDownPatchPlugin, AkashicPointDownEvent, OnPointDown);


