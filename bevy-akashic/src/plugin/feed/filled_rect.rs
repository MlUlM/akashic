use bevy::app::{App, Last};
use bevy::prelude::{Changed, IntoSystemConfigs, Plugin, Query};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::plugin::system_set::AkashicSystemSet;
use crate::prelude::NativeAkashicEntity;
use crate::prelude::object2d::entity::filled_rect::CssColor;

pub struct FilledRectPlugin;


impl Plugin for FilledRectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (
                feed_filled_rects_system
            ).in_set(AkashicSystemSet::Feed));
    }
}


fn feed_filled_rects_system(
    filled_rects: Query<(&CssColor, &NativeAkashicEntity), Changed<CssColor>>,
) {
    for (css_color, native) in filled_rects.iter() {
        feed_filled_rect_properties(
            &native.0,
            css_color.0.clone(),
        );
    }
}


#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(js_name = feedFilledRectProperties)]
    fn feed_filled_rect_properties(entity: &akashic::object2d::entity::AkashicEntity, css_color: String);
}