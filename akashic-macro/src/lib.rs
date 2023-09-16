use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use crate::asset::expand_impl_asset;

use crate::event::{expand_point_delta_event_base, expand_point_event_base};
use crate::object2d::entity::cacheable::expand_cacheable;
use crate::object2d::entity::expand_entity;
use crate::object2d::expand_object_2d;
use crate::param::e_parameter::expand_e_parameter;
use crate::param::object_2d_parameter::expand_object_2d_parameter;
use crate::scene::expand_scene;

mod trigger;
mod scene;
mod children;
mod modified;
mod asset;
mod param;
mod event;
mod object2d;



#[proc_macro_derive(Asset)]
pub fn asset(input: TokenStream) -> TokenStream {
    expand_impl_asset(input)
}


#[proc_macro_derive(Object2D)]
pub fn object_2d(input: TokenStream) -> TokenStream {
    expand_object_2d(input)
}


#[proc_macro_derive(EntityObject2D)]
pub fn akashic_entity(input: TokenStream) -> TokenStream {
    expand_entity(input)
}


#[proc_macro_derive(CacheableEntity)]
pub fn chacheable_entity(input: TokenStream) -> TokenStream {
    expand_cacheable(input)
}


#[proc_macro_derive(AkashicScene)]
pub fn akashic_scene(input: TokenStream) -> TokenStream {
    expand_scene(input)
}


#[proc_macro_derive(PointEventBase)]
pub fn akashic_event_base(input: TokenStream) -> TokenStream {
    expand_point_event_base(input)
}


#[proc_macro_derive(PointDeltaEventBase)]
pub fn akashic_delta_event_base(input: TokenStream) -> TokenStream {
    expand_point_delta_event_base(input)
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