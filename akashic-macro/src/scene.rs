use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

use crate::trigger::{expand_message_trigger, expand_on_load, expand_on_point_down_capture, expand_on_update, expand_point_move_capture, expand_point_up_capture};

#[inline(always)]
pub fn expand_scene(input: TokenStream) -> TokenStream {
    try_expand_scene(input).unwrap_or_else(|e| e.into_compile_error().into())
}


fn try_expand_scene(input: TokenStream) -> syn::Result<TokenStream> {
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let on_load = expand_on_load(&entity_name)?;
    let on_update = expand_on_update(&entity_name)?;
    let on_point_down_capture = expand_on_point_down_capture(&entity_name)?;
    let point_up = expand_point_up_capture(&entity_name)?;
    let point_move = expand_point_move_capture(&entity_name)?;
    let message = expand_message_trigger(&entity_name)?;

    Ok(quote! {
        #on_update
        #on_load
        #on_point_down_capture
        #point_up
        #point_move
        #message

        #[wasm_bindgen(js_namespace=g)]
        extern "C"{
            #[wasm_bindgen(method, getter)]
            pub fn asset(this: &#entity_name) -> crate::prelude::AssetAccessor;

            #[wasm_bindgen(method, getter)]
            pub fn children(this: &#entity_name) -> Box<[crate::prelude::AkashicEntity]>;

            #[wasm_bindgen(method)]
            pub fn modified(this: &#entity_name);
        }
    }
        .into())
}