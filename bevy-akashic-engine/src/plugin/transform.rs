use std::f32::consts::PI;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Changed, Commands, Entity, IntoSystemConfigs, Or, Query, Transform};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::component::entity_size::AkashicEntitySize;
use crate::plugin::modify::RequestModifyTarget;
use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::NativeAkashicEntity;


pub struct AkashicTransformPlugin;

impl Plugin for AkashicTransformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (
                update_transform_system
            ).in_set(AkashicSystemSet::UpdateAkashicEntities));
    }
}


fn update_transform_system(
    mut commands: Commands,
    mut transforms: Query<(
        Entity,
        &NativeAkashicEntity,
        &Transform,
        &AkashicEntitySize,
    ),
        Or<(Changed<Transform>, Changed<AkashicEntitySize>)>
    >
) {
    for (entity, native, transform, size) in transforms.iter_mut() {
        let akashic_entity = native.0.clone();
        let (_, rad) = transform.rotation.to_axis_angle();
        let angle = rad * 180. / PI;

        update_entity_base(
            akashic_entity.clone(),
            transform.translation.x,
            transform.translation.y,
            angle,
            size.x,
            size.y,
        );
        commands.entity(entity).insert(RequestModifyTarget);
    }
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {

    #[wasm_bindgen(js_name = updateEntityBase)]
    fn update_entity_base(entity: akashic_rs::entity::Entity, x: f32, y: f32, angle: f32, width: f32, height: f32);
}