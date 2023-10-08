use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::{Src};
use crate::util::into_js_value::macros::into_js_value;

#[wasm_bindgen(js_namespace = g)]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Surface;
}

into_js_value!(Surface);


#[allow(clippy::from_over_into)]
impl Into<Src> for Surface {
    #[inline]
    fn into(self) -> Src {
        Src::Surface(self)
    }
}
