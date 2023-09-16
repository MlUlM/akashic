use std::f32::consts::PI;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Changed, Commands, Entity, IntoSystemConfigs, Or, Query, Transform};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::component::object2d::anchor::Anchor;

use crate::component::object2d::entity_size::AkashicEntitySize;
use crate::plugin::modify::RequestModifyTarget;
use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::NativeAkashicEntity;
use crate::prelude::object2d::touchable::Touchable;

pub struct AkashicEntityObject2DPlugin;

impl Plugin for AkashicEntityObject2DPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (
                feed_entity_objects
            ).in_set(AkashicSystemSet::Feed));
    }
}


fn feed_entity_objects(
    mut commands: Commands,
    mut transforms: Query<(
        Entity,
        &NativeAkashicEntity,
        &Transform,
        &AkashicEntitySize,
        &Anchor,
        &Touchable
    ),
        Or<(Changed<Transform>, Changed<AkashicEntitySize>, Changed<Anchor>, Changed<Touchable>)>
    >,
) {
    for (entity, native, transform, size, anchor, touchable) in transforms.iter_mut() {
        let akashic_entity = native.0.clone();
        let (_, rad) = transform.rotation.to_axis_angle();
        let angle = rad * 180. / PI;

        feed_entity_properties(
            &akashic_entity,
            transform.translation.x,
            transform.translation.y,
            angle,
            size.x,
            size.y,
            transform.scale.x,
            transform.scale.y,
            anchor.x,
            anchor.y,
            touchable.0
        );

        commands.entity(entity).insert(RequestModifyTarget);
    }
}




#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[wasm_bindgen(js_name = feedEntityProperties)]
    fn feed_entity_properties(
        entity: &akashic_rs::object2d::entity::AkashicEntity,
        x: f32,
        y: f32,
        angle: f32,
        width: f32,
        height: f32,
        scale_x: f32,
        scale_y: f32,
        anchor_x: Option<f32>,
        anchor_y: Option<f32>,
        touchable: bool
    );
}