use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

#[inline]
pub fn expand_point_event_base(input: TokenStream) -> TokenStream {
    try_expand_point_event_base(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


#[inline]
pub fn expand_point_delta_event_base(input: TokenStream) -> TokenStream {
    try_expand_point_delta_event_base(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


fn try_expand_point_event_base(input: TokenStream) -> syn::Result<TokenStream2> {
    let name = syn::parse::<ItemStruct>(input)?.ident;

    Ok(quote! {
           #[wasm_bindgen::prelude::wasm_bindgen(js_namespace=g)]
           extern "C"{
              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = button)]
              fn _button(this: &#name) -> u8;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = eventFlags)]
              fn _event_flags(this: &#name) -> u8;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = local)]
              fn _local(this: &#name) -> bool;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = player)]
              fn _player(this: &#name) -> Option<crate::player::Player>;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = point)]
              fn _point(this: &#name) -> crate::prelude::CommonOffset;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = pointerId)]
              fn _pointer_id(this: &#name) -> f32;

              #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = target)]
              fn _target(this: &#name) -> Option<crate::object2d::entity::AkashicEntity>;
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
                fn target(&self) -> Option<crate::object2d::entity::AkashicEntity>{
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
}


fn try_expand_point_delta_event_base(input: TokenStream) -> syn::Result<TokenStream2> {
    let base = try_expand_point_event_base(input.clone())?;
    let name = syn::parse::<ItemStruct>(input)?.ident;

    Ok(quote! {
        #base

        #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = g)]
        extern {
            #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = startDelta)]
            fn _start_delta(this: &#name) -> crate::prelude::CommonOffset;

            #[wasm_bindgen::prelude::wasm_bindgen(method, getter, js_name = prevDelta)]
            fn _prev_delta(this: &#name) -> crate::prelude::CommonOffset;
        }

        impl crate::trigger::PointDeltaEventBase for #name{
            #[inline(always)]
            fn start_delta(&self) -> crate::prelude::CommonOffset{
                self._start_delta()
            }

            #[inline(always)]
            fn prev_delta(&self) -> crate::prelude::CommonOffset{
                self._prev_delta()
            }
        }
    })
}
