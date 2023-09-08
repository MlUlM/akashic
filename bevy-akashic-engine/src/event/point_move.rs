use bevy::math::Vec2;
use bevy::prelude::Event;

use crate::component::AkashicEntityId;

#[derive(Clone, Debug, Event)]
pub struct PointMoveEvent {
    pub entity_id: AkashicEntityId,
    pub point: Vec2,
    pub start_delta: Vec2,
    pub prev_delta: Vec2,
}


impl PointMoveEvent{
    #[inline(always)]
    pub fn current_pos(&self) -> Vec2{
        self.point + self.start_delta
    }
}


#[derive(Clone, Debug, Event)]
pub struct ScenePointMoveEvent {
    pub point: Vec2,
    pub start_delta: Vec2,
    pub prev_delta: Vec2,
}




