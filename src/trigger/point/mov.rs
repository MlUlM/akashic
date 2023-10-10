use auto_delegate::delegate;

use crate::event::point::mov::PointMoveEvent;
use crate::prelude::Trigger;

#[delegate]
pub trait PointMoveHandler {
    /// Returns the trigger dealing with [`PointMoveEvent`].
    fn on_point_move(&self) -> Trigger<PointMoveEvent>;
}


#[delegate]
pub trait PointMoveCaptureHandler {
    /// Returns the trigger dealing with [`PointMoveEvent`].
    fn on_point_move_capture(&self) -> Trigger<PointMoveEvent>;
}


