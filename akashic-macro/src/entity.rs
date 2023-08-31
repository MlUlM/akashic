use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::trigger::expand_entity_triggers;

#[inline]
pub fn expand_entity(input: TokenStream) -> TokenStream {
    try_expand_entity(input).unwrap_or_else(|e| e.into_compile_error().into())
}


fn try_expand_entity(input: TokenStream) -> syn::Result<TokenStream> {
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let entity = expand_impl_entity(&entity_name);
    let triggers = expand_entity_triggers(&entity_name)?;

    Ok(quote! {
        #entity
        #triggers
    }.into())
}

fn expand_impl_entity(entity_name: &Ident) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn id(this: &#entity_name) -> usize;

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn x(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn y(this: &#entity_name) -> f32;
        }


        impl crate::entity::E for #entity_name{
            #[inline(always)]
            fn as_js_value(&self) -> JsValue{
                self.obj.clone()
            }
        }
    }
}