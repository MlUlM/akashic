use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

use crate::asset::expand_asset;
use crate::children::expand_children;
use crate::modified::expand_modify;
use crate::trigger::{expand_on_load, expand_on_point_down_capture, expand_on_update, expand_point_move_capture, expand_point_up_capture};

#[inline(always)]
pub fn expand_scene(input: TokenStream) -> TokenStream {
    try_expand_scene(input).unwrap_or_else(|e| e.into_compile_error().into())
}


fn try_expand_scene(input: TokenStream) -> syn::Result<TokenStream> {
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let on_load = expand_on_load(&entity_name)?;
    let on_update = expand_on_update(&entity_name)?;
    let children = expand_children(&entity_name);
    let modify = expand_modify(&entity_name);
    let asset = expand_asset(&entity_name);
    let on_point_down_capture = expand_on_point_down_capture(&entity_name)?;
    let point_up = expand_point_up_capture(&entity_name)?;
    let point_move =expand_point_move_capture(&entity_name)?;

    Ok(quote! {
        #on_update
        #on_load
        #children
        #modify
        #asset
        #on_point_down_capture
        #point_up
        #point_move
    }.into())
}