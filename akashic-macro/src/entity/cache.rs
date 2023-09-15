use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

#[inline]
pub fn expand_cacheable(input: TokenStream) -> TokenStream {
    try_expand_cacheable(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


fn try_expand_cacheable(input: TokenStream) -> syn::Result<TokenStream2> {
    let item = syn::parse::<ItemStruct>(input)?;
    let entity_name = item.ident;

    Ok(quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_namespace=g)]
        extern{
            #[wasm_bindgen::prelude::wasm_bindgen(method, js_name=invalidate)]
            fn _invalidate(this: &#entity_name);
        }

        impl crate::entity::Cacheable for #entity_name{
            #[inline(always)]
            fn invalidate(&self){
                self._invalidate();
            }
        }
    })
}