use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::{PointDeltaEventBase};

#[wasm_bindgen]
extern "C" {
    /// ポインティング操作の終了を表すイベント。 PointDownEvent後にのみ発生する。
    ///
    /// PointUpEvent#startDeltaによってPointDownEvent時からの移動量が、 PointUpEvent#prevDeltaによって直近のPointMoveEventからの移動量が取得出来る。 PointUpEvent#pointにはPointDownEvent#pointと同じ値が格納される。
    #[derive(Clone, Debug, PointDeltaEventBase)]
    pub type PointUpEvent;
}