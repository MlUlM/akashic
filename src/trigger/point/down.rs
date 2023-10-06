use auto_delegate::delegate;

use crate::event::point::down::PointDownEvent;
use crate::trigger::Trigger;

#[delegate]
pub trait PointDownHandler {
    /// Returns the trigger dealing with [`PointDownEvent`].
    ///
    /// Fires when point down to the entity that owns this trigger.
    fn on_point_down(&self) -> Trigger<PointDownEvent>;
}


pub trait PointDownCaptureHandler {
    /// Returns the trigger dealing with [`PointDownEvent`].
    /// 
    /// Fires when point down to the scene that owns this trigger.
    fn on_point_down_capture(&self) -> Trigger<PointDownEvent>;
}



