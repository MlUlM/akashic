use auto_delegate::delegate;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::PointEventBase;

use crate::trigger::Trigger;

#[delegate]
pub trait PointDownHandler {
    fn on_point_down(&self) -> Trigger<PointDownEvent>;
}


pub trait PointDownCaptureHandler {
    fn on_point_down_capture(&self) -> Trigger<PointDownEvent>;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, PointEventBase)]
    pub type PointDownEvent;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type CommonOffset;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn x(this: &CommonOffset) -> f32;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn y(this: &CommonOffset) -> f32;
}