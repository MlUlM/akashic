use auto_delegate::delegate;
use crate::prelude::{AkashicEntity, CommonOffset};

pub mod up;
pub mod mov;
pub mod down;


#[delegate]
pub trait PointEventBase {
    fn button(&self) -> i16;

    fn event_flags(&self) -> u8;

    fn local(&self) -> bool;

    fn target(&self) -> Option<AkashicEntity>;

    fn player(&self) -> Option<crate::player::Player>;

    fn point(&self) -> CommonOffset;

    fn pointer_id(&self) -> i32;
}


#[delegate]
pub trait PointDeltaEventBase: PointEventBase {
    fn start_delta(&self) -> CommonOffset;


    fn prev_delta(&self) -> CommonOffset;
}
