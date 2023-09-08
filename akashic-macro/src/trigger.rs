use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;

use crate::ident;

pub fn expand_entity_triggers(entity_name: &Ident) -> syn::Result<TokenStream2> {
    let point_down = expand_trigger(
        entity_name,
        ident("onPointDown"),
        quote!(crate::prelude::PointDownHandler),
        quote!(crate::prelude::PointDownEvent),
    )?;
    let on_update = expand_on_update(entity_name)?;
    let on_load = expand_on_load(entity_name)?;
    let point_up = expand_point_up(entity_name)?;
    let point_move = expand_point_move(entity_name)?;

    Ok(quote! {
        #point_down
        #on_update
        #on_load
        #point_up
        #point_move
    })
}


pub fn expand_on_update(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onUpdate"),
        quote!(crate::prelude::UpdateHandler),
        quote!(crate::prelude::Void),
    )
}


pub fn expand_on_load(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onLoad"),
        quote!(crate::prelude::OnLoadHandler),
        quote!(crate::prelude::Scene),
    )
}

pub fn expand_on_point_down_capture(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onPointDownCapture"),
        quote!(crate::prelude::PointDownCaptureHandler),
        quote!(crate::prelude::PointDownEvent),
    )
}

pub fn expand_point_up_capture(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onPointUpCapture"),
        quote!(crate::trigger::point_up::PointUpCaptureHandler),
        quote!(crate::trigger::point_up::PointUpEvent),
    )
}

pub fn expand_point_up(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onPointUp"),
        quote!(crate::trigger::point_up::PointUpHandler),
        quote!(crate::trigger::point_up::PointUpEvent),
    )
}

pub fn expand_point_move_capture(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onPointMoveCapture"),
        quote!(crate::trigger::point_move::PointMoveCaptureHandler),
        quote!(crate::trigger::point_move::PointMoveEvent),
    )
}

pub fn expand_point_move(entity_name: &Ident) -> syn::Result<TokenStream2> {
    expand_trigger(
        entity_name,
        ident("onPointMove"),
        quote!(crate::trigger::point_move::PointMoveHandler),
        quote!(crate::trigger::point_move::PointMoveEvent),
    )
}


fn expand_trigger(
    entity_name: &Ident,
    fn_name: Ident,
    trigger_path: TokenStream2,
    output: TokenStream2,
) -> syn::Result<TokenStream2> {
    let fn_low_case = Ident::new(&fn_name.to_string().to_case(Case::Snake), Span::call_site());

    Ok(quote! {
        paste::paste!{
            #[wasm_bindgen]
            extern "C"{
                #[wasm_bindgen(js_namespace = g, method, getter, js_name = #fn_name)]
                fn [<_ #fn_low_case>](this: &#entity_name) -> crate::trigger::NativeTrigger;
            }

            impl #trigger_path for #entity_name{
                fn #fn_low_case(&self) -> crate::trigger::Trigger<#output> {
                    crate::trigger::Trigger::new(self.[<_ #fn_low_case>]())
                }
            }
        }
    })
}