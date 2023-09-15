use proc_macro::TokenStream;

use quote::quote;
use syn::{FieldsNamed, ItemStruct};
use syn::__private::TokenStream2;

use crate::param::{expand_custom_setter_field, expand_default_field, expand_snake_case_field, push_if_need, push_if_need_option_number};

#[inline]
pub fn expand_e_parameter(input: TokenStream) -> TokenStream {
    try_expand_e_parameter(input).unwrap_or_else(|e| e.into_compile_error()).into()
}


fn try_expand_e_parameter(
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
    let ident = &item.ident;
    quote! {
        paste::paste!{
            impl [<#ident Builder>]{
                pub fn parent(&mut self, parent: crate::parent::Parent) -> &mut Self{
                    let new = self;
                    new.parent = Some(parent.as_js_value());
                    new
                }
            }
        }
    }
}


fn expand_fields(fields: &mut FieldsNamed) {
    push_if_need(fields, "scene", expand_default_field(quote! {pub scene: crate::prelude::Scene}));
    push_if_need(fields, "local", expand_default_field(quote! {pub local: bool}));
    push_if_need(fields, "parent", expand_custom_setter_field(quote! {pub parent: wasm_bindgen::JsValue}));
    push_if_need(fields, "children", expand_custom_setter_field(quote! {pub children: Box<[wasm_bindgen::JsValue]>}));
    push_if_need(fields, "touchable", expand_default_field(quote! {pub touchable: bool}));
    push_if_need_option_number(fields, "id");
    push_if_need(fields, "tag", expand_default_field(quote! {pub tag: wasm_bindgen::JsValue}));
    push_if_need(fields, "shader_program", expand_default_field(expand_snake_case_field("shaderProgram", quote! {pub shader_program: Option<crate::shader::ShaderProgram>})));
}





