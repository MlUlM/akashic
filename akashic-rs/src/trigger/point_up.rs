use auto_delegate::delegate;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicEventBase;

use crate::prelude::Trigger;

#[delegate]
pub trait PointUpHandler {
    fn on_point_up(&self) -> Trigger<PointUpEvent>;
}


#[delegate]
pub trait PointUpCaptureHandler {
    fn on_point_up_capture(&self) -> Trigger<PointUpEvent>;
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    /// ポインティング操作の終了を表すイベント。 PointDownEvent後にのみ発生する。
    //
    // PointUpEvent#startDeltaによってPointDownEvent時からの移動量が、 PointUpEvent#prevDeltaによって直近のPointMoveEventからの移動量が取得出来る。 PointUpEvent#pointにはPointDownEvent#pointと同じ値が格納される。
    #[derive(Clone, Debug, AkashicEventBase)]
    pub type PointUpEvent;
}

// #[wasm_bindgen(getter_with_clone)]
// pub struct PointUpEvent {
//     pub button: f32,
//     #[wasm_bindgen(js_name = eventFlags)]
//     pub event_flags: f32,
//     pub local: bool,
//     pub player: Option<Player>,
//     pub point: CommonOffset,
//
//     #[wasm_bindgen(js_name = pointerId)]
//     pub pointer_id: f32,
//
//     #[wasm_bindgen(js_name = prevDelta)]
//     pub prev_delta: CommonOffset,
//
//     #[wasm_bindgen(js_name = startDelta)]
//     pub start_delta: CommonOffset,
//
//     pub target: Option<Entity>,
//
//     #[wasm_bindgen(js_name = type)]
//     pub event_type: String,
// }