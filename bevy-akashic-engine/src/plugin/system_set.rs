use bevy::app::{App, Last};
use bevy::prelude::{Plugin, SystemSet};

#[derive(SystemSet, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AkashicSystemSet {
    /// Added new component with [`crate::component::AkashicEntityId`]
    Added,

    PointEvents,

    Feed,

    Despawn,
    
    Modify
}


pub struct AkashicSystemSetPlugin;


impl Plugin for AkashicSystemSetPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(
                Last,
                (
                    AkashicSystemSet::Added,
                    AkashicSystemSet::PointEvents,
                    AkashicSystemSet::Feed,
                    AkashicSystemSet::Despawn,
                    AkashicSystemSet::Modify
                ),
            );
    }
}