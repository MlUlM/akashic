use bevy::math::{Quat, Vec3};
use bevy::prelude::{Bundle, Transform};

use akashic_rs::prelude::EntityObject2D;

use crate::component::{AkashicEntityId, NativeAkashicEntity};
use crate::component::object2d::entity_size::AkashicEntitySize;
use crate::prelude::object2d::anchor::Anchor;
use crate::prelude::object2d::entity::entity_properties;

#[derive(Bundle, Debug)]
pub struct AkashicEntityBundle {
    id: AkashicEntityId,
    size: AkashicEntitySize,
    transform: Transform,
    anchor: Anchor,
    native: NativeAkashicEntity,
}


impl AkashicEntityBundle {
    pub fn new(entity: impl EntityObject2D) -> Self {
        let properties = entity_properties(&entity);
        let id = AkashicEntityId(properties.id);
        let size = AkashicEntitySize::new(&properties);
        let transform = Transform::from_xyz(properties.x, properties.y, 0.)
            .with_rotation(Quat::from_rotation_z(properties.angle))
            .with_scale(Vec3::new(properties.scale_x, properties.scale_y, 0.));
        let anchor = Anchor::new(&properties);
        let native: akashic_rs::object2d::entity::Entity = entity.into();

        Self {
            id,
            size,
            transform,
            anchor,
            native: NativeAkashicEntity(native),
        }
    }
}
