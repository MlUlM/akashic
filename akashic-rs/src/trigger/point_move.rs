use auto_delegate::delegate;
use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::AkashicEventBase;

use crate::prelude::{CommonOffset, Entity, Trigger};

#[delegate]
pub trait PointMoveHandler {
    fn on_point_move(&self) -> Trigger<PointMoveEvent>;
}


#[delegate]
pub trait PointMoveCaptureHandler {
    fn on_point_move_capture(&self) -> Trigger<PointMoveEvent>;
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    /// ポインティング操作の移動を表すイベント。 PointDownEvent後にのみ発生するため、MouseMove相当のものが本イベントとして発生することはない。
    //
    // PointMoveEvent#startDeltaによってPointDownEvent時からの移動量が、 PointMoveEvent#prevDeltaによって直近のPointMoveEventからの移動量が取得出来る。 PointMoveEvent#pointにはPointMoveEvent#pointと同じ値が格納される。
    //
    // 本イベントは、プレイヤーがポインティングデバイスを移動していなくても、 カメラの移動等視覚的にポイントが変化している場合にも発生する。
    #[derive(Clone, Debug, AkashicEventBase)]
    pub type PointMoveEvent;

    #[wasm_bindgen(method, getter, js_name = prevDelta)]
    pub fn prev_delta(this: &PointMoveEvent) -> CommonOffset;

    #[wasm_bindgen(method, getter, js_name = startDelta)]
    pub fn start_delta(this: &PointMoveEvent) -> CommonOffset;
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