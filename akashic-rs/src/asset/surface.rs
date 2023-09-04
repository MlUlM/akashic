use wasm_bindgen::prelude::wasm_bindgen;

use crate::asset::src::{IntoSrc, Src};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Surface;
}


impl IntoSrc for Surface {
    #[inline]
    fn into_src(self) -> Src {
        Src::Surface(self)
    }
}
