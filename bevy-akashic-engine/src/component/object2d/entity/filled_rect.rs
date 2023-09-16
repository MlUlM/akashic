use bevy::prelude::{Bundle, Component, Deref, DerefMut};

use akashic_rs::prelude::FilledRect;

use crate::command::IntoBundle;
use crate::prelude::object2d::entity::entity_bundle::AkashicEntityBundle;

#[derive(Bundle)]
pub struct FilledRectBundle {
    basic: AkashicEntityBundle,
    css_color:  CssColor
}


impl IntoBundle<FilledRectBundle> for FilledRect {
    fn into_bundle(self) -> FilledRectBundle {
        FilledRectBundle {
            css_color: CssColor(self.css_color()),
            basic: AkashicEntityBundle::new(self),
        }
    }
}



#[derive(Component, Eq, PartialEq, Hash, Clone, Deref, DerefMut)]
pub struct CssColor(pub(crate) String);



impl CssColor{
    pub fn set_rgba(&mut self, r: f32, g: f32, b: f32, a: f32){
        let convert = |v: f32|{
            (v * 255.).round() as u8
        };
        self.0 = format!("rgba({},{},{},{a})", convert(r), convert(g), convert(b));
    }
}