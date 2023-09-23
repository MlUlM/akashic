use proc_macro::TokenStream;

use quote::quote;
use syn::{FieldsNamed, ItemStruct};
use syn::__private::TokenStream2;

use crate::param::{expand_custom_setter_field, expand_snake_case_field, push_if_need, push_if_need_expand_option_number_anchor, push_if_need_option_number};

#[inline]
pub fn expand_object2d_params(input: TokenStream) -> TokenStream {
    try_expand_object2d_params(input).unwrap_or_else(|e| e.into_compile_error()).into()
}


fn try_expand_object2d_params(
    input: TokenStream
) -> syn::Result<TokenStream2> {
    let mut item = syn::parse::<ItemStruct>(input)?;
    if let syn::Fields::Named(fields) = &mut item.fields {
        expand_fields(fields);
    }

    Ok(quote!(#item))
}


fn expand_fields(fields: &mut FieldsNamed) {
    push_if_need_option_number(fields, "x");
    push_if_need_option_number(fields, "y");
    push_if_need_option_number(fields, "width");
    push_if_need_option_number(fields, "height");
    push_if_need_option_number(fields, "opacity");
    push_if_need_option_number(fields, "scale_x");
    push_if_need_option_number(fields, "scale_y");
    push_if_need_option_number(fields, "angle");
    push_if_need(fields, "composite_operation", expand_custom_setter_field(expand_snake_case_field("compositeOperation", quote! {pub composite_operation: crate::prelude::OptionNumber})));
    push_if_need_expand_option_number_anchor(fields, "anchor_x");
    push_if_need_expand_option_number_anchor(fields, "anchor_y");
}



