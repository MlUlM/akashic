use std::f32::consts::PI;

use bevy::app::{App, Last, Plugin};
use bevy::prelude::{Changed, IntoSystemConfigs, Or, Query, Res, Transform, Visibility};
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_rs::prelude::AkashicEntity;

use crate::component::object2d::anchor::Anchor;
use crate::component::object2d::entity_size::AkashicEntitySize;
use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::NativeAkashicEntity;
use crate::prelude::object2d::touchable::Touchable;
use crate::resource::game::GameInfo;

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
    game_info: Res<GameInfo>,
    transforms: Query<(
        &NativeAkashicEntity,
        &Transform,
        &AkashicEntitySize,
        &Anchor,
        &Touchable,
        &Visibility
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
        visibility
    ) in transforms.iter() {
        let akashic_entity = native.0.clone();
        let (_, rad) = transform.rotation.to_axis_angle();

        let angle = rad * 180. / PI;

        feed_entity_properties(
            &akashic_entity,
            game_info.half_width() + transform.translation.x,
            game_info.half_height() - transform.translation.y,
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