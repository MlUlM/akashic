use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;


#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct Object2DParameterObject {
    ///このオブジェクトの横位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
    pub x: Option<f32>,
    /**
     * このオブジェクトの縦位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
     * @default 0
     */
    pub y: Option<f32>,
    /**
     * このオブジェクトの横幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
     * @default 0
     */
    pub width: Option<f32>,
    /**
     * このオブジェクトの縦幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
     * @default 0
     */
    pub height: Option<f32>,
    /**
     * 0～1でオブジェクトの不透明度を表す。
     * この値が0の場合、Rendererは描画処理を省略する。
     * @default 1
     */
    pub opacity: Option<f32>,
    /**
     * オブジェクトの横方向の倍率。
     * @default 1
     */
    pub scaleX: Option<f32>,
    /**
     * オブジェクトの縦方向の倍率。
     * @default 1
     */
    pub scaleY: Option<f32>,
    /**
     * オブジェクトの回転。度数で指定する。
     * @default 0
     */
    pub angle: Option<f32>,
    /**
     * 描画時の合成方法を指定する。
     * 省略された場合、合成方法を指定しない（親の合成方法を利用する）。
     * なお `CompositeOperation` での指定は非推奨である。 `CompositeOperationString` を利用すること。
     * @default undefined
     */
    pub compositeOperation: Option<JsString>,

    pub anchorX: Option<f32>,

    pub anchorY: Option<f32>,
}


