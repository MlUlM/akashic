use bevy::app::{App, Last};
use bevy::prelude::{Changed, IntoSystemConfigs, Plugin, Query};
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_rs::prelude::AkashicEntity;
use crate::component::NativeAkashicEntity;

use crate::component::text::AkashicText;
use crate::plugin::system_set::AkashicSystemSet;

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
    labels: Query<(&AkashicText, &NativeAkashicEntity), Changed<AkashicText>>,
) {
    for (text, native) in labels.iter() {
        feed_label_properties(
            &native.0,
            text.text.clone(),
            text.style.text_align.into(),
            text.style.text_color.clone().map(|text| text.into()),
            text.style.width_auto_adjust,
        );
    }
}

#[wasm_bindgen(js_namespace = g)]
extern {
    /// ## Notes
    ///
    /// この関数はアカシックエンジンに組み込まれているものではなく、ビルドの際に自動で追加されるものです。
    #[wasm_bindgen(js_name = feedLabelProperties)]
    fn feed_label_properties(entity: &AkashicEntity, text: String, text_align: String, text_color: Option<String>, width_auto_adjust: bool);
}