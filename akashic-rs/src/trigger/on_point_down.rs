use wasm_bindgen::prelude::wasm_bindgen;

use crate::trigger::Trigger;

pub trait PointDownHandler {
    fn on_point_down(&self) -> Trigger<PointDownEvent>;
}

#[macro_export]
macro_rules! on_point_down {
    ($entity_name: ident) => {
        $crate::trigger!($entity_name, "onPointDown", on_point_down, $crate::trigger::prelude::PointDownHandler, $crate::trigger::on_point_down::PointDownEvent);
    };
}


#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type PointDownEvent;

    #[wasm_bindgen(js_namespace = g, getter, method)]
    pub fn point(this: &PointDownEvent) -> CommonOffset;
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