use auto_delegate::delegate;
use crate::event::point::up::PointUpEvent;
use crate::prelude::Trigger;

#[delegate]
pub trait PointUpHandler {
    fn on_point_up(&self) -> Trigger<PointUpEvent>;
}


#[delegate]
pub trait PointUpCaptureHandler {
    fn on_point_up_capture(&self) -> Trigger<PointUpEvent>;
}




