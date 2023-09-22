pub mod entity;


pub trait Object2D {
    /// オブジェクトのアンカーの横位置。アンカーについては以下の通り。
    ///
    /// アンカーとして設定した箇所がこのオブジェクトの基点 (位置、拡縮・回転の基点) となる。
    /// 単位は相対値 (左上端が (0, 0) 中央が (0.5, 0,5) 右下端が (1,1) ) である。 初期値は 0 である。 E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    ///
    /// ## Notes
    ///
    /// anchorX または anchorY のどちらを明示的に null に指定した場合、 このオブジェクトのアンカーは前バージョン(v2.x.x 以前)のデフォルトの挙動 (位置 x, y は左上端を基準に、拡大・縮小・回転の基点は中央を基準に決定) と同様になる。 これは前バージョンとの後方互換性のために存在する。
    fn anchor_x(&self) -> Option<f32>;


    /// オブジェクトのアンカーの横位置を指定する。アンカーについては以下の通り。
    ///
    /// アンカーとして設定した箇所がこのオブジェクトの基点 (位置、拡縮・回転の基点) となる。
    /// 単位は相対値 (左上端が (0, 0) 中央が (0.5, 0,5) 右下端が (1,1) ) である。 初期値は 0 である。 E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    ///
    /// ## Notes
    ///
    /// anchorX または anchorY のどちらを明示的に null に指定した場合、 このオブジェクトのアンカーは前バージョン(v2.x.x 以前)のデフォルトの挙動 (位置 x, y は左上端を基準に、拡大・縮小・回転の基点は中央を基準に決定) と同様になる。 これは前バージョンとの後方互換性のために存在する。
    fn set_anchor_x(&self, x: Option<f32>);


    /// オブジェクトのアンカーの縦位置。アンカーについては以下の通り。
    ///
    /// アンカーとして設定した箇所がこのオブジェクトの基点 (位置、拡縮・回転の基点) となる。
    /// 単位は相対値 (左上端が (0, 0) 中央が (0.5, 0,5) 右下端が (1,1) ) である。 初期値は 0 である。 E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    ///
    /// ## Notes
    ///
    /// anchorX または anchorY のどちらを明示的に null に指定した場合、 このオブジェクトのアンカーは前バージョン(v2.x.x 以前)のデフォルトの挙動 (位置 x, y は左上端を基準に、拡大・縮小・回転の基点は中央を基準に決定) と同様になる。 これは前バージョンとの後方互換性のために存在する。
    fn anchor_y(&self) -> Option<f32>;


    /// オブジェクトのアンカーの縦位置を指定する。アンカーについては以下の通り。
    ///
    /// アンカーとして設定した箇所がこのオブジェクトの基点 (位置、拡縮・回転の基点) となる。
    /// 単位は相対値 (左上端が (0, 0) 中央が (0.5, 0,5) 右下端が (1,1) ) である。 初期値は 0 である。 E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    ///
    /// ## Notes
    ///
    /// anchorX または anchorY のどちらを明示的に null に指定した場合、 このオブジェクトのアンカーは前バージョン(v2.x.x 以前)のデフォルトの挙動 (位置 x, y は左上端を基準に、拡大・縮小・回転の基点は中央を基準に決定) と同様になる。 これは前バージョンとの後方互換性のために存在する。
    fn set_anchor_y(&self, y: Option<f32>);


    /// オブジェクトの回転。度数 (時計回り) で指定する。 初期値は 0 である。
    fn angle(&self) -> f32;


    /// オブジェクトの回転(時計回りの度数)を変更する。
    ///
    /// E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_angle(&self, angle: f32);


    /// このオブジェクトの横幅。 初期値は 0 である。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
    fn width(&self) -> f32;


    /// このオブジェクトの横幅を変更する。
    ///
    /// E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_width(&self, width: f32);


    /// このオブジェクトの縦幅。 初期値は 0 である。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
    fn height(&self) -> f32;


    /// このオブジェクトの縦幅を変更する。
    ///
    /// E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_height(&self, height: f32);


    /// オブジェクトの横方向の倍率。 初期値は 1 である。
    fn scale_x(&self) -> f32;


    /// オブジェクトの横方向の倍率を変更する。
    ///
    /// E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_scale_x(&self, scale_x: f32);


    /// オブジェクトの縦方向の倍率。 初期値は 1 である。
    fn scale_y(&self) -> f32;


    /// オブジェクトの縦方向の倍率を変更する。
    ///
    /// E や Camera2D においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_scale_y(&self, scale_y: f32);


    /// 0～1でオブジェクトの不透明度を表す。 初期値は 1 である。本値が0の場合、Rendererは描画処理を省略する。
    fn opacity(&self) -> f32;


    /// オブジェクトの不透明度を変更する。値は0~1の範囲。
    ///
    /// E においてこの値を変更した場合、 modified() を呼び出す必要がある。
    fn set_opacity(&self, opacity: f32);


    /// このオブジェクトの横位置。 初期値は 0 である。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
    fn x(&self) -> f32;


    /// このオブジェクトの縦位置を変更する。
    /// E や Camera2D において変更した場合、 modified() を呼び出す必要がある。
    fn set_x(&self, x: f32);


    /// このオブジェクトの縦位置。 初期値は 0 である。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
    fn y(&self) -> f32;


    /// このオブジェクトの縦位置を変更する。
    /// E や Camera2D において変更した場合、 modified() を呼び出す必要がある。
    fn set_y(&self, y: f32);


    /// オブジェクトのアンカーの位置を設定する。 このメソッドは anchorX と anchorY を同時に設定するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある。
    fn anchor(&self, x: f32, y: f32);


    /// オブジェクトを相対的に移動する。 このメソッドは x と y を同時に加算するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある。
    fn move_by(&self, x: f32, y: f32);


    /// オブジェクトを移動する。 このメソッドは x と y を同時に設定するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある。
    fn move_to(&self, x: f32, y: f32);


    /// オブジェクトのサイズを相対的に変更する。 このメソッドは width と height を同時に加算するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある
    fn resize_by(&self, width: f32, height: f32);


    /// オブジェクトのサイズを設定する。 このメソッドは width と height を同時に設定するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある。
    fn resize_to(&self, width: f32, height: f32);


    /// オブジェクトの拡大率を設定する。 このメソッドは scaleX と scaleY に同じ値を同時に設定するためのユーティリティメソッドである。
    ///
    /// E や Camera2D においてこのメソッドを呼び出した場合、 modified() を呼び出す必要がある。
    fn scale(&self, scale: f32);
}
