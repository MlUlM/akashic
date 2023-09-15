use bevy::prelude::Component;
use akashic_rs::entity::label::{TextColor, TextAlignment};

#[derive(Component, Debug, Eq, PartialEq)]
pub struct AkashicText{
    pub text: String,
    pub style: AkashicTextStyle
}


#[derive(Component, Debug, Eq, PartialEq)]
pub struct AkashicTextStyle{
    pub text_color: Option<TextColor>,
    pub font_size: isize,
    pub width_auto_adjust: bool,
    pub text_align: TextAlignment,
}


