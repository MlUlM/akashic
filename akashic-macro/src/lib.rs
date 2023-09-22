use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

use crate::asset::expand_impl_asset;
use crate::cacheable::derive::expand_cacheable_derive;
use crate::entity::derive::expand_entity_derive;
use crate::entity::params::expand_entity_params;

use crate::event::{expand_point_delta_event_base, expand_point_event_base};
use crate::object2d::derive::expand_object_2d_derive;

use crate::object2d::param::expand_object_2d_params;

use crate::scene::expand_scene;

mod trigger;
mod scene;
mod asset;
mod param;
mod event;
mod object2d;
mod entity;
mod cacheable;


#[proc_macro_derive(Asset)]
pub fn asset(input: TokenStream) -> TokenStream {
    expand_impl_asset(input)
}


#[proc_macro_derive(Object2D)]
pub fn object_2d(input: TokenStream) -> TokenStream {
    expand_object_2d_derive(input)
}


#[proc_macro_derive(EntityObject2D)]
pub fn akashic_entity(input: TokenStream) -> TokenStream {
    expand_entity_derive(input)
}


#[proc_macro_derive(CacheableEntity)]
pub fn chacheable_entity(input: TokenStream) -> TokenStream {
    expand_cacheable_derive(input)
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
pub fn object_2d_params(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_object_2d_params(input)
}


#[proc_macro_attribute]
pub fn object_e_parameter(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_entity_params(expand_object_2d_params(input))
}


#[inline(always)]
pub(crate) fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
