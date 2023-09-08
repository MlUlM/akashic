use bevy::math::Vec3;
use bevy::prelude::Event;

use crate::component::AkashicEntityId;

#[derive(Clone, Debug, Event)]
pub struct PointMoveEvent {
    pub entity_id: AkashicEntityId,
    pub point: Vec3,
    pub start_delta: Vec3,
    pub prev_delta: Vec3,
}


#[derive(Clone, Debug, Event)]
pub struct ScenePointMoveEvent {
    pub point: Vec3,
    pub start_delta: Vec3,
    pub prev_delta: Vec3,
}




