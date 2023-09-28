use bevy::app::App;
use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity, Event, EventReader, NonSendMut, Plugin, Query, With};
use bevy::window::PrimaryWindow;
use crate::input::akashic_pointer::down::AkashicPointDownPlugin;

pub mod down;


pub struct AkashicPointerPlugins;


impl Plugin for AkashicPointerPlugins{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AkashicPointDownPlugin);
    }
}



#[derive(Deref, DerefMut)]
pub(crate) struct AkashicPontEventStorage<E>(Vec<E>);


impl<E> Default for AkashicPontEventStorage<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

pub(crate) fn push_all_point_down_event<E: Event + Clone>(
    mut er: EventReader<E>,
    mut storage: NonSendMut<AkashicPontEventStorage<E>>
){
    for event in er.iter(){
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