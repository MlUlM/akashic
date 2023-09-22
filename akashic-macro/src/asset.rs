use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;




#[inline]
pub fn expand_impl_asset(input: TokenStream) -> TokenStream {
    try_expand_impl_asset(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


fn try_expand_impl_asset(input: TokenStream) -> syn::Result<TokenStream2> {
    let name = syn::parse::<ItemStruct>(input)?.ident;

    Ok(quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_namespace=g)]
        extern {
            #[wasm_bindgen(method, getter, js_name=id)]
            fn _id(this: &#name) -> String;

            #[wasm_bindgen(method, getter, js_name=originalPath)]
            fn _original_path(this: &#name) -> String;

            #[wasm_bindgen(method, getter, js_name=path)]
            fn _path(this: &#name) -> String;

            #[wasm_bindgen(method, getter, js_name="type")]
            fn _asset_type(this: &#name) -> String;

            #[wasm_bindgen(method,  js_name=destroy)]
            fn _destroy(this: &#name);

            #[wasm_bindgen(method, getter, js_name=destroyed)]
            fn _destroyed(this: &#name) -> bool;

            #[wasm_bindgen(method, getter, js_name=inUse)]
            fn _in_use(this: &#name) -> bool;
        }


        impl crate::asset::Asset for #name{
            #[inline(always)]
            fn id(&self) -> String{
                self._id()
            }

            #[inline(always)]
            fn original_path(&self) -> String{
                self._original_path()
            }


            #[inline(always)]
            fn path(&self) -> String{
                self._path()
            }

            #[inline(always)]
            fn asset_type(&self) -> String{
                self._asset_type()
            }

            #[inline(always)]
            fn destroy(&self) {
                self._destroy()
            }

            #[inline(always)]
            fn destroyed(&self) -> bool{
                self._destroyed()
            }


            #[inline(always)]
            fn in_use(&self) -> bool{
                self._in_use()
            }
        }
    })
}