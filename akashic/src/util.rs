use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsValue;

pub(crate) mod into_js_value;


pub(crate) trait FunctionIntoJsValue {
    /// ボクシングされたクロージャーからJsValueに変換します。
    fn into_js_value(self) -> JsValue;
}


impl FunctionIntoJsValue for Box<dyn FnMut()>
{
    fn into_js_value(self) -> JsValue {
        let cb = Closure::wrap(self);
        let ret = cb.as_ref().clone();
        cb.forget();
        ret
    }
}


impl<I, O> FunctionIntoJsValue for Box<dyn FnMut(I) -> O>
    where I: FromWasmAbi + 'static,
          O: IntoWasmAbi + 'static
{
    fn into_js_value(self) -> JsValue {
        let cb = Closure::wrap(self);
        let ret = cb.as_ref().clone();
        cb.forget();
        ret
    }
}


