use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

use crate::entity::expand_entity;
use crate::event::expand_event_base;
use crate::param::e_parameter::expand_e_parameter;
use crate::param::object_2d_parameter::expand_object_2d_parameter;
use crate::scene::expand_scene;

mod trigger;
mod entity;
mod scene;
mod children;
mod modified;
mod asset;
mod param;
mod event;

#[proc_macro_derive(AkashicEntity)]
pub fn akashic_entity(input: TokenStream) -> TokenStream {
    expand_entity(input)
}


#[proc_macro_derive(AkashicScene)]
pub fn akashic_scene(input: TokenStream) -> TokenStream {
    expand_scene(input)
}


#[proc_macro_derive(AkashicEventBase)]
pub fn akashic_event_base(input: TokenStream) -> TokenStream {
    expand_event_base(input)
}


#[proc_macro_attribute]
pub fn object_2d_parameter(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_object_2d_parameter(input)
}


#[proc_macro_attribute]
pub fn object_e_parameter(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_e_parameter(expand_object_2d_parameter(input))
}

#[proc_macro_derive(EParamSetters)]
pub fn object_e_setter(input: TokenStream) -> TokenStream {
    param::e_parameter::expand_param_setters(input)
}





#[inline(always)]
pub(crate) fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}