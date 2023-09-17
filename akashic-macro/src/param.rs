
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;
use syn::FieldsNamed;
use syn::parse::Parser;

pub mod object_2d_parameter;
pub mod e_parameter;


#[inline]
pub(crate) fn push_if_need(fields: &mut FieldsNamed, field_name: &str, token: TokenStream2) {
    let exists = fields.named.iter().any(|f| {
        f.ident.as_ref().is_some_and(|name| *name == field_name)
    });

    if !exists {
        fields.named.push(syn::Field::parse_named.parse2(token).unwrap());
    }
}


#[inline]
pub(crate) fn push_if_need_option_number(fields: &mut FieldsNamed, field_name: &str) {
    push_if_need(fields, field_name, expand_option_number(field_name));
}


#[inline]
pub(crate) fn push_if_need_expand_option_number_anchor(fields: &mut FieldsNamed, field_name: &str) {
    push_if_need(fields, field_name, expand_option_number_anchor(field_name));
}


#[inline]
pub fn expand_snake_case_field(js_name: &str, token: TokenStream2) -> TokenStream2 {
    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name)]
        #token
    }
}


#[inline]
pub fn expand_option_number(field_name: &str) -> TokenStream2 {
    let js_name  =  field_name.to_case(Case::Camel);
    let js_name_ref = js_name.as_str();
    let field_name = Ident::new(field_name, Span::call_site());

    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name=#js_name_ref)]
        #[builder(setter(into, strip_option), default)]
        pub #field_name: crate::option_number::OptionNumber
    }
}



#[inline]
pub fn expand_option_number_anchor(field_name: &str) -> TokenStream2 {
    let js_name  =  field_name.to_case(Case::Camel);
    let js_name_ref = js_name.as_str();
    let field_name = Ident::new(field_name, Span::call_site());

    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name=#js_name_ref)]
        #[builder(setter(into, strip_option), default)]
        pub #field_name: crate::option_number::OptionNumber
    }
}


#[inline]
pub fn expand_custom_setter_field(token: TokenStream2) -> TokenStream2 {
    quote! {
        #[builder(setter(custom), default)]
        #token
    }
}


#[inline]
pub fn expand_default_field(token: TokenStream2) -> TokenStream2 {
    quote! {
        #[builder(default)]
        #token
    }
}

