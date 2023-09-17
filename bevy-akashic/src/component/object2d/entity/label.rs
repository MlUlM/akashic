use bevy::prelude::{Bundle, Component};

use akashic_rs::object2d::entity::cacheable::label::Label;

use crate::command::IntoBundle;
use crate::component::text::{AkashicText, AkashicTextStyle};
use crate::prelude::object2d::entity::entity_bundle::AkashicEntityBundle;

#[derive(Debug, Bundle)]
pub struct AkashicLabelBundle {
    basic: AkashicEntityBundle,
    text: AkashicText,
    native_label: NativeAkashicLabel
}


impl IntoBundle<AkashicLabelBundle> for Label {
    fn into_bundle(self) -> AkashicLabelBundle {
        AkashicLabelBundle {
            text: AkashicText {
                text: self.text(),
                style: AkashicTextStyle {
                    text_color: self.text_color(),
                    font_size: 0,
                    width_auto_adjust: false,
                    text_align: self.text_align(),
                },
            },
            basic: AkashicEntityBundle::new(self.clone()),
            native_label: NativeAkashicLabel(self)
        }
    }
}


#[derive(Component, Debug)]
pub(crate) struct NativeAkashicLabel(pub Label);


unsafe impl Sync for NativeAkashicLabel {}

unsafe impl Send for NativeAkashicLabel {}