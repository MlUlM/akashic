use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;
use crate::children::expand_children;
use crate::modified::expand_modify;

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
    let children = expand_children(entity_name);
    let modify = expand_modify(entity_name);
    quote! {
        #modify
        #children

        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method, getter, js_name=id)]
            fn _id(this: &#entity_name) -> usize;

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn x(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, setter = x)]
            pub fn set_x(this: &#entity_name, x: f32);

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn y(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, setter = y)]
            pub fn set_y(this: &#entity_name, y: f32);
        }

        impl crate::entity::E for #entity_name{
            #[inline(always)]
            fn id(&self) -> usize{
                self._id()
            }

            #[inline(always)]
            fn as_js_value(&self) -> JsValue{
                self.obj.clone()
            }
        }
    }
}