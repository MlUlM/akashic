use bevy::app::App;
use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity, Event, EventReader, NonSendMut, Plugin, Query, With};
use bevy::window::PrimaryWindow;

use crate::input::akashic_pointer::down::AkashicPointDownPatchPlugin;
use crate::input::akashic_pointer::r#move::AkashicPointMovePatchPlugin;
use crate::input::akashic_pointer::up::AkashicPointUpPatchPlugin;

pub mod down;

pub mod r#move;
pub mod up;
mod raycast;
mod rapier;

pub struct AkashicPointPatchPlugins;


impl Plugin for AkashicPointPatchPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AkashicPointDownPatchPlugin,
            AkashicPointMovePatchPlugin,
            AkashicPointUpPatchPlugin
        ));
    }
}


#[derive(Deref, DerefMut)]
pub(crate) struct AkashicPontEventStorage<E>(Vec<E>);


impl<E> Default for AkashicPontEventStorage<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}


#[macro_use]
pub(crate) mod macros {
    macro_rules! akashic_pointer_plugin {
        ($plugin_name: ident, $event: ident, $component: ident) => {
            pub struct $plugin_name;

            impl bevy::prelude::Plugin for $plugin_name {
                fn build( & self, app: & mut bevy::prelude::App) {
                    use bevy::prelude::IntoSystemConfigs;

                    app
                        .insert_non_send_resource(crate::input::akashic_pointer::AkashicPontEventStorage::<$event>::default())
                        .add_systems(bevy::prelude::PreUpdate, (
                            crate::input::akashic_pointer::push_all_point_down_event::<$event>,
                            crate::input::akashic_pointer::rapier::update_hit_rapiers::<$event, $component>,
                            crate::input::akashic_pointer::raycast::update_hit_raycasts::<$event, $component>,
                            crate::input::akashic_pointer::insert_all_remaining_events_to_window::<$event, $component>
                        ).chain());

                    #[cfg(feature="picking")]
                    {
                        app
                            .add_event::<bevy_mod_picking::events::Pointer<$event>>()
                            .add_plugins(bevy_mod_picking::prelude::EventListenerPlugin::<bevy_mod_picking::events::Pointer<$event>>::default());
                    }
                }
            }
        }
    }

    pub(crate) use akashic_pointer_plugin;
}

pub(crate) fn push_all_point_down_event<E: Event + Clone>(
    mut er: EventReader<E>,
    mut storage: NonSendMut<AkashicPontEventStorage<E>>,
) {
    for event in er.iter() {
        storage.push(event.clone());
    }
}


pub(crate) fn insert_all_remaining_events_to_window<E: Event, C: From<E> + Component>(
    mut commands: Commands,
    mut storage: NonSendMut<AkashicPontEventStorage<E>>,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    let Ok(window_entity) = window.get_single() else { return; };
    while let Some(event) = storage.pop() {
        commands.entity(window_entity).insert(C::from(event));
    }
}