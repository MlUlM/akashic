use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::entity::derive::try_expand_entity_derive;

#[inline]
pub fn expand_cacheable_derive(input: TokenStream) -> TokenStream {
    try_expand_cacheable(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


fn try_expand_cacheable(input: TokenStream) -> syn::Result<TokenStream2> {
    let entity = try_expand_entity_derive(input.clone())?;
    let item = syn::parse::<ItemStruct>(input)?;
    let entity_name = item.ident;

    Ok(quote! {
        #entity

        #[wasm_bindgen::prelude::wasm_bindgen(js_namespace=g)]
        extern{
            #[wasm_bindgen::prelude::wasm_bindgen(method, js_name=invalidate)]
            fn _invalidate(this: &#entity_name);
        }

        impl crate::object2d::entity::cacheable::CacheableEntityObject2D for #entity_name{
            #[inline(always)]
            fn invalidate(&self){
                self._invalidate();
            }
        }
    })
}