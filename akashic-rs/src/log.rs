use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($message:tt)*) => {
        $crate::prelude::log(&format!($($message)*));
    };
}
