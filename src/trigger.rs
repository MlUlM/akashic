use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod load;
pub mod update;
pub mod message;
pub mod join;
pub mod point;


pub mod prelude {
    pub use crate::trigger::{
        load::*,
        message::MessageHandler,
        point::{
            down::*,
            mov::*,
            up::*,
        },
        Trigger,
        update::*,
        Void,
    };
}


#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, Default)]
pub struct Void;

#[macro_export]
macro_rules! trigger {
    ($entity_name: ident, $js_name: literal, $fn_name: ident, $trigger_path: path,  $event: path) => {
        paste::paste!{
             #[wasm_bindgen]
            extern "C"{
                    #[wasm_bindgen(js_namespace = g, method, getter, js_name = $js_name)]
                   fn [<_ $fn_name>](this: &$entity_name) -> $crate::trigger::NativeTrigger;
            }

            impl $trigger_path for $entity_name{
                fn $fn_name(&self) -> $crate::trigger::Trigger<$event> {
                    $crate::trigger::Trigger::new(self.[<_ $fn_name>]())
                }
            }
        }
    };

    ($entity_name: ident, $js_name: literal,  $fn_name: ident, $trigger_path: path) => {
        $crate::trigger!($entity_name, $js_name, $fn_name, $trigger_path, $crate::trigger::Void);
    }
}


pub struct Trigger<T>(pub(crate) NativeTrigger, PhantomData<T>);


impl<T> Trigger<T> {
    pub fn new(native: NativeTrigger) -> Trigger<T> {
        Self(native, PhantomData)
    }
}


impl<T: FromWasmAbi + 'static> Trigger<T> {
    pub fn add(&self, f: impl FnMut(T) + 'static) {
        let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut(T)>);
        let ret = cb.as_ref().clone();
        cb.forget();
        self.0._add(ret)
    }
}


impl Trigger<Void> {
    pub fn add(&self, f: impl FnMut() + 'static) {
        let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
        let ret = cb.as_ref().clone();
        cb.forget();
        self.0._add(ret)
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Trigger")]
    pub type NativeTrigger;

    #[wasm_bindgen(js_name = "add", method)]
    fn _add(this: &NativeTrigger, f: JsValue);
}


