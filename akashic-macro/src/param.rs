use proc_macro2::{Ident, Span};
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use syn::{FieldsNamed, ItemStruct};
use syn::__private::TokenStream2;
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
pub fn expand_snake_case_field(js_name: &str, token: TokenStream2) -> TokenStream2 {
    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name)]
        #token
    }
}


#[inline]
pub fn expand_option_number(field_name: &str) -> TokenStream2 {
    let field_name = Ident::new(field_name, Span::call_site());
    expand_custom_setter_field(quote!(pub #field_name: crate::param::OptionNumber))
}


#[inline]
pub fn expand_custom_setter_field(token: TokenStream2) -> TokenStream2 {
    quote! {
        #[builder(setter(custom))]
        #token
    }
}


pub fn expand_option_number_setter(item: &ItemStruct, field_name: &str) -> Option<TokenStream2> {
    let field_name = Ident::new(field_name, Span::call_site());

    let was_override = item.fields.iter().any(|f| {
        let ty = &f.ty;
        quote!(#ty).to_string() == "f32"
    });

    if was_override {
        None
    } else {
        Some(quote! {
             pub fn #field_name(&mut self, num: f32) -> &mut Self{
                let new = self;
                new.#field_name = Some(crate::param::OptionNumber::from(num));
                new
             }
        })
    }
}