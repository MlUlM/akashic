use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;
use crate::trigger::{expand_on_load, expand_on_update};

#[inline(always)]
pub fn expand_scene(input: TokenStream) -> TokenStream {
    try_expand_scene(input).unwrap_or_else(|e|e.into_compile_error().into())
}


fn try_expand_scene(input: TokenStream) -> syn::Result<TokenStream>{
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let on_load = expand_on_load(&entity_name)?;
    let on_update = expand_on_update(&entity_name)?;
    Ok(quote!{
        #on_update
        #on_load
    }.into())
}