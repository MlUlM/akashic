use auto_delegate::delegate;

use crate::event::point::point_move::PointMoveEvent;
use crate::prelude::Trigger;

#[delegate]
pub trait PointMoveHandler {
    fn on_point_move(&self) -> Trigger<PointMoveEvent>;
}


#[delegate]
pub trait PointMoveCaptureHandler {
    fn on_point_move_capture(&self) -> Trigger<PointMoveEvent>;
}


