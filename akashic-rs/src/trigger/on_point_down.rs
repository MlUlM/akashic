use auto_delegate::delegate;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::player::Player;

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
    #[derive(Clone)]
    pub type PointDownEvent;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn point(this: &PointDownEvent) -> CommonOffset;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn player(this: &PointDownEvent) -> Player;
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