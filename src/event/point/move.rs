use wasm_bindgen::prelude::wasm_bindgen;
use akashic_macro::PointDeltaEventBase;

#[wasm_bindgen]
extern "C" {
    /// ポインティング操作の移動を表すイベント。 PointDownEvent後にのみ発生するため、MouseMove相当のものが本イベントとして発生することはない。
    ///
    /// PointMoveEvent#startDeltaによってPointDownEvent時からの移動量が、 PointMoveEvent#prevDeltaによって直近のPointMoveEventからの移動量が取得出来る。 PointMoveEvent#pointにはPointMoveEvent#pointと同じ値が格納される。
    ///
    /// 本イベントは、プレイヤーがポインティングデバイスを移動していなくても、 カメラの移動等視覚的にポイントが変化している場合にも発生する。
    #[derive(Clone, PointDeltaEventBase)]
    pub type PointMoveEvent;
}



