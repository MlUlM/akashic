use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Bundle, Transform, TransformBundle, Visibility};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_rs::prelude::{AkashicEntity, EntityObject2D};

use crate::command::IntoBundle;
use crate::component::{AkashicEntityId, NativeAkashicEntity};
use crate::component::object2d::entity_size::AkashicEntitySize;
use crate::component::object2d::touchable::Touchable;
use crate::prelude::object2d::anchor::Anchor;

#[derive(Bundle, Debug)]
pub struct AkashicEntityBundle {
    id: AkashicEntityId,
    size: AkashicEntitySize,
    transform: TransformBundle,
    anchor: Anchor,
    touchable: Touchable,
    native: NativeAkashicEntity,
    visibility: Visibility,
}


impl AkashicEntityBundle {
    pub fn new(entity: impl EntityObject2D) -> Self {
        let properties = entity_properties(&entity);
        let id = AkashicEntityId(properties.id);
        let size = AkashicEntitySize::new(Vec2::new(properties.width, properties.height));
        let transform = TransformBundle::from_transform(Transform::from_xyz(properties.x, properties.y, 0.)
            .with_rotation(Quat::from_rotation_z(properties.angle))
            .with_scale(Vec3::new(properties.scale_x, properties.scale_y, 0.)));
        let anchor = Anchor::new(properties.anchor_x, properties.anchor_y);
        let touchable = Touchable(properties.touchable);
        let visibility = if properties.visible { Visibility::Visible } else { Visibility::Hidden };
        let native: AkashicEntity = entity.into();

        Self {
            id,
            size,
            transform,
            anchor,
            touchable,
            visibility,
            native: NativeAkashicEntity(native),
        }
    }
}


impl IntoBundle<AkashicEntityBundle> for akashic_rs::object2d::entity::AkashicEntity {
    #[inline(always)]
    fn into_bundle(self) -> AkashicEntityBundle {
        AkashicEntityBundle::new(self)
    }
}


#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(js_name = getEntityProperties)]
    fn _entity_properties(entity: &JsValue) -> JsValue;
}

#[inline]
fn entity_properties(entity: &impl EntityObject2D) -> EntityProperties {
    let raw = _entity_properties(entity.js_value_ref());
    serde_wasm_bindgen::from_value(raw).unwrap()
}


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct EntityProperties {
    pub id: isize,

    pub x: f32,

    pub y: f32,

    pub width: f32,

    pub height: f32,

    pub angle: f32,

    #[serde(rename = "scaleX")]
    pub scale_x: f32,

    #[serde(rename = "scaleY")]
    pub scale_y: f32,

    #[serde(rename = "anchorX")]
    pub anchor_x: Option<f32>,

    #[serde(rename = "anchorY")]
    pub anchor_y: Option<f32>,

    pub touchable: bool,

    pub visible: bool,
}