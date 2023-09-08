use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn expand_event_base(input: TokenStream) -> TokenStream {
     syn::parse::<ItemStruct>(input)
        .map(|item|item.ident)
        .map(|name|quote!{
           #[wasm_bindgen(js_namespace=g)]
           extern "C"{
              #[wasm_bindgen(method, getter)]
              pub fn button(this: &#name) -> u8;

              #[wasm_bindgen(method, getter, js_name = eventFlags)]
              pub fn event_flags(this: &#name) -> u8;

              #[wasm_bindgen(method, getter)]
              pub fn local(this: &#name) -> bool;

              #[wasm_bindgen(method, getter)]
              pub fn player(this: &#name) -> Option<crate::player::Player>;

              #[wasm_bindgen(method, getter)]
              pub fn point(this: &#name) -> crate::prelude::CommonOffset;

              #[wasm_bindgen(method, getter)]
              pub fn pointer_id(this: &#name) -> f32;

              #[wasm_bindgen(method, getter)]
              pub fn target(this: &#name) -> Option<crate::entity::Entity>;
           }
        })
         .unwrap_or_else(|e|e.into_compile_error())
         .into()
}