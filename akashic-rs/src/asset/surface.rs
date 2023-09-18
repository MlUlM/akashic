use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;

use crate::asset::src::{Src};
use crate::util::into_js_value::macros::into_js_value;

#[wasm_bindgen(js_namespace=g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Surface;

    #[wasm_bindgen(method, getter, js_name="_drawable")]
    pub fn canvas(this: &Surface) -> HtmlCanvasElement;
}

into_js_value!(Surface);


#[allow(clippy::from_over_into)]
impl Into<Src> for Surface {
    #[inline]
    fn into(self) -> Src {
        Src::Surface(self)
    }
}
