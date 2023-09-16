use auto_delegate::delegate;

use crate::event::point::point_down::PointDownEvent;
use crate::trigger::Trigger;

#[delegate]
pub trait PointDownHandler {
    fn on_point_down(&self) -> Trigger<PointDownEvent>;
}


pub trait PointDownCaptureHandler {
    fn on_point_down_capture(&self) -> Trigger<PointDownEvent>;
}



