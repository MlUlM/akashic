use wasm_bindgen::closure::WasmClosureFnOnce;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::audio::AudioAsset;
use crate::asset::image::ImageAsset;
use crate::asset::text::TextAsset;
use crate::util::FunctionIntoJsValue;

pub mod surface;
pub mod src;
pub mod image;
pub mod audio;
pub mod text;

pub trait Asset {
    fn id(&self) -> String;


    fn original_path(&self) -> String;


    fn path(&self) -> String;


    fn asset_type(&self) -> String;


    /// このアセットのリソースの破棄を行う。
    fn destroy(&self);


    /// このアセットのリソースが破棄済みであるかどうかを判定する。
    fn destroyed(&self) -> bool;


    /// 現在利用中で解放出来ない Asset かどうかを返す。 戻り値は、利用中である場合真、でなければ偽である。
    ///
    /// 本メソッドは通常 false が返るべきである。 例えば Sprite の元画像として使われているケース等では、その Sprite によって Asset は Surface に変換されているべきで、 Asset が利用中で解放出来ない状態になっていない事を各プラットフォームで保障する必要がある。
    ///
    /// 唯一、例外的に本メソッドが true を返すことがあるのは音楽を表す Asset である。 BGM等はシーンをまたいで演奏することもありえる上、 演奏中のリソースのコピーを常に各プラットフォームに強制するにはコストがかかりすぎるため、 本メソッドは true を返し、適切なタイミングで Asset が解放されるよう制御する必要がある。
    fn in_use(&self) -> bool;
}


#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone)]
    pub type AssetAccessor;
}


#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(method, js_name = getImageById)]
    pub fn get_image_by_id(this: &AssetAccessor, asset_id: String) -> ImageAsset;

    #[wasm_bindgen(method, js_name = getAllImages)]
    pub fn get_all_images(this: &AssetAccessor) -> Box<[ImageAsset]>;

    #[wasm_bindgen(method, js_name = getAllImages)]
    pub fn get_all_images_with_path_pattern(this: &AssetAccessor, path_pattern: String) -> Box<[ImageAsset]>;

    #[wasm_bindgen(method, js_name = getAllImages)]
    fn _get_all_images_with_filter(this: &AssetAccessor, filter: JsValue) -> Box<[ImageAsset]>;
}


#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(method, js_name = getAudioById)]
    pub fn get_audio_by_id(this: &AssetAccessor, asset_id: String) -> AudioAsset;

    #[wasm_bindgen(method, js_name = getAllAudios)]
    pub fn get_all_audios(this: &AssetAccessor) -> Box<[AudioAsset]>;

    #[wasm_bindgen(method, js_name = getAllAudios)]
    pub fn get_all_audios_with_path_pattern(this: &AssetAccessor, path_pattern: String) -> Box<[AudioAsset]>;

    #[wasm_bindgen(method, js_name = getAllAudios)]
    fn _get_all_audios_with_filter(this: &AssetAccessor, filter: JsValue) -> Box<[AudioAsset]>;
}


#[wasm_bindgen(js_namespace = g)]
extern {
    #[wasm_bindgen(method, js_name = getTextById)]
    pub fn get_text_by_id(this: &AssetAccessor, asset_id: String) -> TextAsset;

    #[wasm_bindgen(method, js_name = getAllTexts)]
    pub fn get_all_texts(this: &AssetAccessor) -> Box<[TextAsset]>;

    #[wasm_bindgen(method, js_name = getAllTexts)]
    pub fn get_all_texts_with_path_pattern(this: &AssetAccessor, path_pattern: String) -> Box<[TextAsset]>;

    #[wasm_bindgen(method, js_name = getAllTexts)]
    fn _get_all_texts_with_filter(this: &AssetAccessor, filter: JsValue) -> Box<[TextAsset]>;
}


impl AssetAccessor {
    #[inline]
    pub fn get_all_images_with_filter(&self, filter: impl 'static + FnMut(String) -> bool) -> Box<[ImageAsset]> {
        self._get_all_images_with_filter((Box::new(filter) as Box<dyn FnMut(String) -> bool>).into_js_value())
    }


    #[inline]
    pub fn get_all_audios_with_filter(&self, filter: impl FnMut(String) -> bool + 'static) -> Box<[AudioAsset]> {
        self._get_all_audios_with_filter((Box::new(filter) as Box<dyn FnMut(String) -> bool>).into_js_function())
    }


    #[inline]
    pub fn get_all_texts_with_filter(&self, filter: impl FnMut(String) -> bool + 'static) -> Box<[TextAsset]> {
        self._get_all_texts_with_filter((Box::new(filter) as Box<dyn FnMut(String) -> bool>).into_js_function())
    }
}












