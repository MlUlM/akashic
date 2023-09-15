use bevy::prelude::{Bundle, Component, Transform};

use akashic_rs::entity::label::Label;
use akashic_rs::prelude::E;

use crate::command::AsBundle;
use crate::component::entity_size::AkashicEntitySize;
use crate::component::NativeAkashicEntity;
use crate::prelude::AkashicEntityId;
use crate::prelude::text::{AkashicText, AkashicTextStyle};

#[derive(Debug, Bundle)]
pub struct AkashicLabelBundle {
    id: AkashicEntityId,
    transform: Transform,
    size: AkashicEntitySize,
    native: NativeAkashicEntity,
    native_label: NativeAkashicLabel,
    text: AkashicText,
}


impl AsBundle<AkashicLabelBundle> for Label {
    fn as_bundle(&self) -> AkashicLabelBundle {
        let size = AkashicEntitySize::new(self);
        let transform = Transform::from_xyz(self.x(), self.y(), 0.);

        AkashicLabelBundle {
            id: AkashicEntityId(self.id()),
            transform,
            size,
            native: NativeAkashicEntity::new(self),
            native_label: NativeAkashicLabel(self.clone()),
            text: AkashicText {
                text: self.text(),
                style: AkashicTextStyle {
                    text_color: self.text_color(),
                    font_size: 0,
                    width_auto_adjust: false,
                    text_align: self.text_align(),
                },
            },
        }
    }
}


#[derive(Component, Debug)]
pub(crate) struct NativeAkashicLabel(pub Label);


unsafe impl Sync for NativeAkashicLabel {}

unsafe impl Send for NativeAkashicLabel {}