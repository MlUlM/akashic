use bevy::prelude::Component;


#[non_exhaustive]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Anchor {
    pub x: Option<f32>,
    pub y: Option<f32>,
}


impl Anchor {
    #[inline]
    pub(crate) fn new(x: Option<f32>, y: Option<f32>) -> Self {
        Self {
            x,
            y,
        }
    }
}