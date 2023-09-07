use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::{FieldsNamed, ItemStruct};
use syn::__private::TokenStream2;

use crate::param::{expand_custom_setter_field, expand_option_number, expand_option_number_setter, expand_snake_case_field, push_if_need, push_if_need_option_number};

#[inline]
pub fn expand_object_2d_parameter(input: TokenStream) -> TokenStream {
    try_expand_object_2d_parameter(input).unwrap_or_else(|e| e.into_compile_error()).into()
}


fn try_expand_object_2d_parameter(
    input: TokenStream
) -> syn::Result<TokenStream2> {
    let mut item = syn::parse::<ItemStruct>(input)?;
    if let syn::Fields::Named(fields) = &mut item.fields {
        expand_fields(fields);
    }

    Ok(quote!(#item))
}

pub(crate) fn expand_param_setters(
    input: TokenStream
) -> TokenStream {
    syn::parse::<ItemStruct>(input)
        .map(|item| expand_setters(&item).into())
        .unwrap_or_else(|e| e.into_compile_error().into())
}

fn expand_setters(
    item: &ItemStruct
) -> TokenStream2 {
    let x = expand_option_number_setter(item, "x");
    let y = expand_option_number_setter(item, "y");
    let width = expand_option_number_setter(item, "width");
    let height = expand_option_number_setter(item, "height");
    let opacity = expand_option_number_setter(item, "opacity");
    let scale_x = expand_option_number_setter(item, "scale_x");
    let scale_y = expand_option_number_setter(item, "scale_y");
    let composite_operation = expand_option_number_setter(item, "composite_operation");
    let anchor_x = expand_option_number_setter(item, "anchor_x");
    let anchor_y = expand_option_number_setter(item, "anchor_y");
    let ident = &item.ident;

    quote! {
        paste::paste!{
            impl [<#ident Builder>]{
                #x
                #y
                #width
                #height
                #opacity
                #scale_x
                #scale_y
                #composite_operation
                #anchor_x
                #anchor_y
            }
        }

    }
}

fn expand_fields(fields: &mut FieldsNamed) {
    push_if_need_option_number(fields, "x");
    push_if_need_option_number(fields, "y");
    push_if_need_option_number(fields, "width");
    push_if_need_option_number(fields, "height");
    push_if_need_option_number(fields, "opacity");
    push_if_need_option_number(fields, "scale_x");
    push_if_need_option_number(fields, "scale_y");
    push_if_need(fields, "composite_operation", expand_custom_setter_field(expand_snake_case_field("compositeOperation", quote! {pub composite_operation: crate::param::OptionNumber})));
    push_if_need_option_number(fields, "anchor_x");
    push_if_need_option_number(fields, "anchor_y");
}



