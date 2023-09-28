use bevy::app::{App, PreUpdate};
use bevy::prelude::{Plugin, IntoSystemConfigs};

use bevy_akashic::event::point_down::OnPointDown;
use bevy_akashic::plugin::event::AkashicPointDownEvent;

use crate::input::akashic_pointer::{AkashicPontEventStorage, insert_all_remaining_events_to_window, push_all_point_down_event};

pub struct AkashicPointDownPlugin;


impl Plugin for AkashicPointDownPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_non_send_resource(AkashicPontEventStorage::<AkashicPointDownEvent>::default())
            .add_systems(PreUpdate, (
                push_all_point_down_event::<AkashicPointDownEvent>,
                insert_all_remaining_events_to_window::<AkashicPointDownEvent, OnPointDown>
            ).chain());
    }
}


