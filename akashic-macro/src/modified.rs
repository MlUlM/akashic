use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;

pub fn expand_modify(entity_name: &Ident) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method)]
            pub fn modified(this: &#entity_name);
        }
    }
}