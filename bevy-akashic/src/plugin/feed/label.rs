use bevy::app::{App, Last};
use bevy::prelude::{Changed, Commands, Entity, IntoSystemConfigs, Plugin, Query};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::plugin::modify::RequestInvalidateTarget;
use crate::plugin::system_set::AkashicSystemSet;
use crate::component::text::AkashicText;
use crate::prelude::NativeAkashicEntity;

pub struct AkashicLabelPlugin;


impl Plugin for AkashicLabelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (
                feed_labels_system
            ).in_set(AkashicSystemSet::Feed));
    }
}


fn feed_labels_system(
    mut commands: Commands,
    labels: Query<(Entity, &AkashicText, &NativeAkashicEntity), Changed<AkashicText>>,
) {
    for (entity, text, native) in labels.iter() {
        feed_label_properties(
            &native.0,
            text.text.clone(),
            text.style.text_align.into(),
            text.style.text_color.clone().map(|text| text.into()),
            text.style.width_auto_adjust,
        );

        commands.entity(entity).insert(RequestInvalidateTarget);
    }
}

#[wasm_bindgen(js_namespace = g)]
extern {
    /// ## Notes
    ///
    /// この関数はアカシックエンジンに組み込まれているものではなく、ビルドの際に自動で追加されるものです。
    #[wasm_bindgen(js_name = feedLabelProperties)]
    fn feed_label_properties(entity: &akashic_rs::object2d::entity::AkashicEntity, text: String, text_align: String, text_color: Option<String>, width_auto_adjust: bool);
}