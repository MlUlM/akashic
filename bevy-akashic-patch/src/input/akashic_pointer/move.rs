use bevy_akashic::plugin::event::AkashicPointMoveEvent;
use bevy_akashic::prelude::point::r#move::OnPointMove;
use crate::input::akashic_pointer::macros::akashic_pointer_plugin;


akashic_pointer_plugin!(AkashicPointMovePatchPlugin, AkashicPointMoveEvent, OnPointMove);
