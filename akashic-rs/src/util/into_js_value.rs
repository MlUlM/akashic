use wasm_bindgen::JsValue;

pub trait IntoJsValue{
    fn into_js_value(self) -> JsValue;
}


#[macro_use]
pub mod macros{
    macro_rules! into_js_value {
        ($name: ident) => {
            impl crate::util::into_js_value::IntoJsValue for $name{
                #[inline]
                fn into_js_value(self) -> wasm_bindgen::JsValue{
                    self.obj
                }
            }
        };
    }

    pub(crate) use into_js_value;
}