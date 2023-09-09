use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

pub fn expand_event_base(input: TokenStream) -> TokenStream {
    syn::parse::<ItemStruct>(input)
        .map(|item| item.ident)
        .map(|name| quote! {
           #[wasm_bindgen(js_namespace=g)]
           extern "C"{
              #[wasm_bindgen(method, getter, js_name = button)]
              fn _button(this: &#name) -> u8;

              #[wasm_bindgen(method, getter, js_name = eventFlags)]
              fn _event_flags(this: &#name) -> u8;

              #[wasm_bindgen(method, getter, js_name = local)]
              fn _local(this: &#name) -> bool;

              #[wasm_bindgen(method, getter, js_name = player)]
              fn _player(this: &#name) -> Option<crate::player::Player>;

              #[wasm_bindgen(method, getter, js_name = point)]
              fn _point(this: &#name) -> crate::prelude::CommonOffset;

              #[wasm_bindgen(method, getter, js_name = pointerId)]
              fn _pointer_id(this: &#name) -> f32;

              #[wasm_bindgen(method, getter, js_name = target)]
              fn _target(this: &#name) -> Option<crate::entity::Entity>;
            }

            impl crate::trigger::PointEventBase for #name{
                #[inline(always)]
                fn button(&self) -> u8{
                    self._button()
                }

                #[inline(always)]
                fn event_flags(&self) -> u8{
                    self._event_flags()
                }

                #[inline(always)]
                fn local(&self) -> bool{
                    self._local()
                }

                #[inline(always)]
                fn target(&self) -> Option<crate::entity::Entity>{
                    self._target()
                }

                #[inline(always)]
                fn player(&self) -> Option<crate::player::Player>{
                    self._player()
                }

                #[inline(always)]
                fn point(&self) -> crate::prelude::CommonOffset{
                    self._point()
                }

                #[inline(always)]
                fn pointer_id(&self) -> f32{
                    self._pointer_id()
                }
            }
        })
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}