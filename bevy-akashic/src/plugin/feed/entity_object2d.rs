use std::f32::consts::PI;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Changed, IntoSystemConfigs, Or, Query, Transform, Visibility};
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_rs::console_log;

use akashic_rs::prelude::AkashicEntity;

use crate::component::object2d::anchor::Anchor;
use crate::component::object2d::entity_size::AkashicEntitySize;
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
    transforms: Query<(
        &NativeAkashicEntity,
        &Transform,
        &AkashicEntitySize,
        &Anchor,
        &Touchable,
        &Visibility,
    ),
        Or<(
            Changed<Transform>,
            Changed<AkashicEntitySize>,
            Changed<Anchor>,
            Changed<Touchable>,
            Changed<Visibility>
        )>
    >,
) {
    for (
        native,
        transform,
        size,
        anchor,
        touchable,
        visibility,
    ) in transforms.iter() {
        let akashic_entity = native.0.clone();

        let (axis, rad) = transform.rotation.normalize().to_axis_angle();


        let rad = axis.z * rad;

        let angle = rad / (2. * PI) * 360.;

        feed_entity_properties(
            &akashic_entity,
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
            angle,
            size.x,
            size.y,
            transform.scale.x,
            transform.scale.y,
            anchor.x,
            anchor.y,
            touchable.0,
            matches!(visibility, Visibility::Visible),
        );
    }
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[wasm_bindgen(js_name = feedEntityProperties)]
    fn feed_entity_properties(
        entity: &AkashicEntity,
        x: f32,
        y: f32,
        z: f32,
        angle: f32,
        width: f32,
        height: f32,
        scale_x: f32,
        scale_y: f32,
        anchor_x: Option<f32>,
        anchor_y: Option<f32>,
        touchable: bool,
        visible: bool,
    );
}