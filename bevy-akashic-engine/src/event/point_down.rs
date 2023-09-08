use bevy::math::Vec2;
use bevy::prelude::Event;

use crate::component::AkashicEntityId;

#[derive(Clone, Debug, Event)]
pub struct ScenePointDown {
    pub point: Vec2,
}

#[derive(Clone, Debug, Event)]
pub struct PointDown {
    pub entity_id: AkashicEntityId,
    pub point: Vec2,
}


