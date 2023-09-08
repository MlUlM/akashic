use bevy::math::Vec2;
use bevy::prelude::Event;

use crate::component::AkashicEntityId;

#[derive(Clone, Debug, Event)]
pub struct PointUpEvent {
    pub entity_id: Option<AkashicEntityId>,
    pub point: Vec2,
}


#[derive(Clone, Debug, Event)]
pub struct ScenePointUpEvent {
    pub point: Vec2,
}




