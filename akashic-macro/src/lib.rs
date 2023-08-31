use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

use crate::entity::expand_entity;
use crate::scene::expand_scene;

mod trigger;
mod entity;
mod scene;
mod children;
mod modified;

#[proc_macro_derive(AkashicEntity)]
pub fn akashic_entity(input: TokenStream) -> TokenStream {
    expand_entity(input)
}


#[proc_macro_derive(AkashicScene)]
pub fn akashic_scene(input: TokenStream) -> TokenStream {
    expand_scene(input)
}


#[inline(always)]
pub(crate) fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}