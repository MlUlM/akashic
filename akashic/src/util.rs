use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsValue;
use crate::prelude::Void;

pub(crate) mod into_js_value;


pub(crate) trait FunctionIntoJsValue<I, O> {
    /// ボクシングされたクロージャーからJsValueに変換します。
    fn into_js_value(self) -> JsValue;
}


impl FunctionIntoJsValue<(), ()> for Box<dyn FnMut()>
{
    fn into_js_value(self) -> JsValue {
        let cb = Closure::wrap(self);
        let ret = cb.as_ref().clone();
        cb.forget();
        ret
    }
}


impl<O, F> FunctionIntoJsValue<Void, O> for F
    where
        F: FnMut() -> O + 'static,
        O: IntoWasmAbi + 'static
{
    fn into_js_value(self) -> JsValue {
        let cb = Closure::wrap(Box::new(self) as Box::<dyn FnMut() -> O>);
        let ret = cb.as_ref().clone();
        cb.forget();
        ret
    }
}


impl<I, O, F> FunctionIntoJsValue<I, O> for F
    where
        F: FnMut(I) -> O + 'static,
        I: FromWasmAbi + 'static,
        O: IntoWasmAbi + 'static
{
    fn into_js_value(self) -> JsValue {
        let cb = Closure::wrap(Box::new(self) as Box::<dyn FnMut(I) -> O>);
        let ret = cb.as_ref().clone();
        cb.forget();
        ret
    }
}


